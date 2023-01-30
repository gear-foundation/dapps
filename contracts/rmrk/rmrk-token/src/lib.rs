#![no_std]

use gstd::{debug, msg, prelude::*, prog::ProgramGenerator, ActorId};
use resource_io::{InitResource, ResourceAction, ResourceEvent};
use rmrk_io::*;
use types::primitives::{BaseId, CollectionAndToken, PartId, TokenId};
mod burn;
mod checks;
mod children;
mod equippable;
mod messages;
mod transfer;
use messages::*;
mod mint;
mod multiresource;
use multiresource::*;
mod utils;
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Default)]
struct RMRKToken {
    name: String,
    symbol: String,
    admin: ActorId,
    token_approvals: HashMap<TokenId, HashSet<ActorId>>,
    rmrk_owners: HashMap<TokenId, RMRKOwner>,
    pending_children: HashMap<TokenId, HashSet<CollectionAndToken>>,
    accepted_children: HashMap<TokenId, HashSet<CollectionAndToken>>,
    children_status: HashMap<CollectionAndToken, ChildStatus>,
    balances: HashMap<ActorId, TokenId>,
    multiresource: MultiResource,
    resource_id: ActorId,
    equipped_tokens: HashSet<TokenId>,
}

static mut RMRK: Option<RMRKToken> = None;

impl RMRKToken {
    // reply about root_owner
    async fn root_owner(&self, token_id: TokenId) {
        let root_owner = self.find_root_owner(token_id).await;
        msg::reply(RMRKEvent::RootOwner(root_owner), 0)
            .expect("Error in reply [RMRKEvent::RootOwner]");
    }

    // internal search for root owner
    async fn find_root_owner(&self, token_id: TokenId) -> ActorId {
        let rmrk_owner = self
            .rmrk_owners
            .get(&token_id)
            .expect("RMRK: Token does not exist");
        if rmrk_owner.token_id.is_some() {
            get_root_owner(&rmrk_owner.owner_id, rmrk_owner.token_id.unwrap()).await
        } else {
            rmrk_owner.owner_id
        }
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let config: InitRMRK = msg::load().expect("Unable to decode InitRMRK");

    let mut rmrk = RMRKToken {
        name: config.name,
        symbol: config.symbol,
        admin: msg::source(),
        ..RMRKToken::default()
    };
    if let Some(resource_hash) = config.resource_hash {
        let (_message_id, resource_id) = ProgramGenerator::create_program(
            resource_hash.into(),
            InitResource {
                resource_name: config.resource_name,
            }
            .encode(),
            0,
        )
        .expect("Error in creating program");
        rmrk.resource_id = resource_id;
        debug!("PROGRAM RESOURCE ID {:?}", resource_id);
        msg::reply(RMRKEvent::ResourceInited { resource_id }, 0).unwrap();
    }
    RMRK = Some(rmrk);
}

#[gstd::async_main]
async unsafe fn main() {
    let action: RMRKAction = msg::load().expect("Could not load msg");
    let rmrk = unsafe { RMRK.get_or_insert(Default::default()) };
    match action {
        RMRKAction::MintToNft {
            parent_id,
            parent_token_id,
            token_id,
        } => {
            rmrk.mint_to_nft(&parent_id, parent_token_id, token_id)
                .await
        }
        RMRKAction::MintToRootOwner {
            root_owner,
            token_id,
        } => rmrk.mint_to_root_owner(&root_owner, token_id),
        RMRKAction::Transfer { to, token_id } => rmrk.transfer(&to, token_id).await,
        RMRKAction::TransferToNft {
            to,
            destination_id,
            token_id,
        } => rmrk.transfer_to_nft(&to, destination_id, token_id).await,
        RMRKAction::Approve { to, token_id } => rmrk.approve(&to, token_id).await,
        RMRKAction::AddChild {
            parent_token_id,
            child_token_id,
        } => rmrk.add_child(parent_token_id, child_token_id).await,
        RMRKAction::AcceptChild {
            parent_token_id,
            child_contract_id,
            child_token_id,
        } => {
            rmrk.accept_child(parent_token_id, child_contract_id, child_token_id)
                .await
        }
        RMRKAction::AddAcceptedChild {
            parent_token_id,
            child_token_id,
        } => {
            rmrk.add_accepted_child(parent_token_id, child_token_id)
                .await
        }
        RMRKAction::TransferChild {
            from,
            to,
            child_token_id,
        } => rmrk.transfer_child(from, to, child_token_id).await,
        RMRKAction::RejectChild {
            parent_token_id,
            child_contract_id,
            child_token_id,
        } => {
            rmrk.reject_child(parent_token_id, child_contract_id, child_token_id)
                .await
        }
        RMRKAction::RemoveChild {
            parent_token_id,
            child_contract_id,
            child_token_id,
        } => {
            rmrk.remove_child(parent_token_id, child_contract_id, child_token_id)
                .await
        }
        RMRKAction::BurnChild {
            parent_token_id,
            child_token_id,
        } => rmrk.burn_child(parent_token_id, child_token_id),
        RMRKAction::BurnFromParent {
            child_token_id,
            root_owner,
        } => rmrk.burn_from_parent(child_token_id, &root_owner).await,
        RMRKAction::Burn(token_id) => rmrk.burn(token_id).await,
        RMRKAction::RootOwner(token_id) => rmrk.root_owner(token_id).await,
        RMRKAction::AddResourceEntry {
            resource_id,
            resource,
        } => rmrk.add_resource_entry(resource_id, resource).await,
        RMRKAction::AddResource {
            token_id,
            resource_id,
            overwrite_id,
        } => rmrk.add_resource(token_id, resource_id, overwrite_id).await,
        RMRKAction::AcceptResource {
            token_id,
            resource_id,
        } => rmrk.accept_resource(token_id, resource_id).await,
        RMRKAction::RejectResource {
            token_id,
            resource_id,
        } => rmrk.reject_resource(token_id, resource_id).await,
        RMRKAction::SetPriority {
            token_id,
            priorities,
        } => rmrk.set_priority(token_id, priorities).await,
        RMRKAction::Equip {
            token_id,
            resource_id,
            equippable,
            equippable_resource_id,
        } => {
            rmrk.equip(token_id, resource_id, equippable, equippable_resource_id)
                .await
        }
        RMRKAction::CheckEquippable {
            parent_token_id,
            child_token_id,
            resource_id,
            slot_id,
        } => {
            rmrk.check_equippable(parent_token_id, child_token_id, resource_id, slot_id)
                .await
        }
        RMRKAction::CheckSlotResource {
            token_id,
            resource_id,
            base_id,
            slot_id,
        } => {
            rmrk.check_slot_resource(token_id, resource_id, base_id, slot_id)
                .await
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let rmrk = unsafe { RMRK.as_ref().expect("RMRK is not initialized") };
    let rmrk_state: RMRKState = rmrk.into();
    msg::reply(rmrk_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
