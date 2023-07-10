#![no_std]

use catalog_io::{CatalogError, CatalogReply};
use equippable::Assets;
use gstd::{exec, msg, prelude::*, ActorId, MessageId};

use primitive_types::U256;
use rmrk_io::*;
use types::primitives::{CollectionAndToken, PartId, TokenId};
mod burn;
mod checks;
mod children;
mod equippable;
mod messages;
mod transfer;
use messages::*;
mod mint;
mod utils;
use hashbrown::{HashMap, HashSet};

pub mod tx_manager;
use tx_manager::TxManager;

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
    balances: HashMap<ActorId, U256>,
}

static mut RMRK: Option<RMRKToken> = None;
static mut ASSETS: Option<Assets> = None;
static mut TX_MANAGER: Option<TxManager> = None;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TxState {
    #[default]
    Initial,
    MsgGetRootOwnerSent,
    ReplyRootOwnerReceived,
    MsgGetNewRootOwnerSent,
    ReplyNewRootOwnerReceived,
    MsgAddChildSent,
    ReplyAddChildReceived,
    MsgBurnChildSent,
    ReplyOnBurnChildReceived,
    MsgTransferChildSent,
    ReplyOnTransferChildReceived,
    MsgBurnFromParentSent,
    ReplyOnBurnFromParentReceived,
    MsgAddResourceSent,
    ReplyOnAddResourceReceived,
    MsgGetResourceSent,
    ReplyOnGetResourceReceived,
    MsgCheckEquippableSent,
    ReplyCheckEquippableReceived,
    MsgCanTokenBeEquippedSent,
    ReplyCanTokenBeEquippedReceived,
    Completed,
    Error(RMRKError),
    CheckMsgSourceAccount(ActorId),
    MsgSourceAccountChecked,
}

#[derive(Clone, Debug)]
pub enum MintToNft {
    Initial,
    MsgGetRootOwnerSent,
    ReplyRootOwnerReceived,
    MsgAddChildSent,
    ReplyAddChildReceived,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Tx {
    msg: RMRKAction,
    state: TxState,
    data: Option<Vec<u8>>,
    processing_msg_payload: Option<Vec<u8>>,
}

impl RMRKToken {
    // reply about root_owner
    fn root_owner(
        &self,
        tx_manager: &mut TxManager,
        token_id: TokenId,
    ) -> Result<RMRKReply, RMRKError> {
        let root_owner = self.get_root_owner(tx_manager, token_id)?;
        Ok(RMRKReply::RootOwner(root_owner))
    }

    fn get_root_owner(
        &self,
        tx_manager: &mut TxManager,
        token_id: TokenId,
    ) -> Result<ActorId, RMRKError> {
        let state = tx_manager.get_state(msg::id());
        let rmrk_owner = self.get_rmrk_owner(token_id)?;
        match state {
            TxState::Initial => {
                if let Some(parent_token_id) = rmrk_owner.token_id {
                    let msg_id = get_root_owner_msg(&rmrk_owner.owner_id, parent_token_id);
                    tx_manager.set_tx_state(TxState::MsgGetRootOwnerSent, msg_id);
                    exec::wait_for(5);
                } else {
                    let root_owner = rmrk_owner.owner_id;
                    Ok(root_owner)
                }
            }
            TxState::ReplyRootOwnerReceived => {
                let root_owner = tx_manager.get_decoded_data::<ActorId>()?;
                Ok(root_owner)
            }
            TxState::Error(error) => Err(error),
            _ => {
                unreachable!()
            }
        }
    }

    fn check_root_owner(
        &self,
        tx_manager: &mut TxManager,
        token_id: TokenId,
    ) -> Result<ActorId, RMRKError> {
        let state = tx_manager.get_state(msg::id());
        let rmrk_owner = self.get_rmrk_owner(token_id)?;

        match state {
            TxState::Initial => {
                if let Some(parent_token_id) = rmrk_owner.token_id {
                    let msg_id = get_root_owner_msg(&rmrk_owner.owner_id, parent_token_id);
                    tx_manager.set_tx_state(TxState::CheckMsgSourceAccount(msg::source()), msg_id);
                    exec::wait_for(5);
                } else {
                    let root_owner = rmrk_owner.owner_id;
                    Ok(root_owner)
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitRMRK = msg::load().expect("Unable to decode InitRMRK");
    let tx_manager: TxManager = Default::default();
    let rmrk = RMRKToken {
        name: config.name,
        symbol: config.symbol,
        admin: msg::source(),
        ..RMRKToken::default()
    };
    let assets: Assets = Default::default();
    unsafe {
        RMRK = Some(rmrk);
        TX_MANAGER = Some(tx_manager);
        ASSETS = Some(assets);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: RMRKAction = msg::load().expect("Could not load msg");
    let rmrk = unsafe { RMRK.as_mut().expect("The contract is not initialized") };

    let assets = unsafe { ASSETS.as_mut().expect("The contract is not initialized") };
    let tx_manager = unsafe { TX_MANAGER.as_mut().expect("Tx manager is not initialized") };
    let reply = process_reply(&action, tx_manager, rmrk, assets);
    msg::reply(reply, 0).expect("Failed to send a reply");
}

fn process_reply(
    action: &RMRKAction,
    tx_manager: &mut TxManager,
    rmrk: &mut RMRKToken,
    assets: &mut Assets,
) -> Result<RMRKReply, RMRKError> {
    match action.clone() {
        RMRKAction::MintToNft {
            parent_id,
            parent_token_id,
            token_id,
        } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
            }

            rmrk.mint_to_nft(tx_manager, (parent_id, parent_token_id, token_id))
        }
        RMRKAction::MintToRootOwner {
            root_owner,
            token_id,
        } => rmrk.mint_to_root_owner(&root_owner, token_id),
        RMRKAction::Transfer { to, token_id } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.transfer(tx_manager, &to, token_id)
        }
        RMRKAction::TransferToNft {
            to,
            destination_id,
            token_id,
        } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.transfer_to_nft(tx_manager, &to, destination_id, token_id)
        }
        RMRKAction::Approve { to, token_id } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_root_owner(tx_manager, rmrk, token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.approve(&to, token_id)
        }
        RMRKAction::AddChild {
            parent_token_id,
            child_token_id,
        } => rmrk.add_child(parent_token_id, child_token_id),
        RMRKAction::AcceptChild {
            parent_token_id,
            child_contract_id,
            child_token_id,
        } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, parent_token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.accept_child(parent_token_id, child_contract_id, child_token_id)
        }
        RMRKAction::TransferChild {
            from,
            to,
            child_token_id,
        } => rmrk.transfer_child(from, to, child_token_id),
        RMRKAction::RejectChild {
            parent_token_id,
            child_contract_id,
            child_token_id,
        } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, parent_token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.remove_or_reject_child(
                tx_manager,
                parent_token_id,
                child_contract_id,
                child_token_id,
                ChildStatus::Pending,
            )
        }
        RMRKAction::RemoveChild {
            parent_token_id,
            child_contract_id,
            child_token_id,
        } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, parent_token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.remove_or_reject_child(
                tx_manager,
                parent_token_id,
                child_contract_id,
                child_token_id,
                ChildStatus::Accepted,
            )
        }
        RMRKAction::BurnChild {
            parent_token_id,
            child_token_id,
        } => rmrk.burn_child(parent_token_id, child_token_id),
        RMRKAction::BurnFromParent { child_token_id } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
            }
            rmrk.burn_from_parent(tx_manager, child_token_id)
        }
        RMRKAction::Burn(token_id) => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, token_id)?;
            }
            tx_manager.check_for_error()?;
            rmrk.burn(tx_manager, token_id)
        }
        RMRKAction::RootOwner(token_id) => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
            }
            tx_manager.check_for_error()?;
            rmrk.root_owner(tx_manager, token_id)
        }

        RMRKAction::Equip {
            token_id,
            child_token_id,
            child_id,
            asset_id,
            slot_part_id,
            child_asset_id,
        } => {
            if tx_manager.tx_does_not_exist() {
                tx_manager.set_tx(action);
                check_approved_account(tx_manager, rmrk, token_id)?;
            }
            tx_manager.check_for_error()?;
            assets.equip(
                tx_manager,
                token_id,
                child_token_id,
                &child_id,
                asset_id,
                slot_part_id,
                child_asset_id,
            )
        }

        RMRKAction::AddEquippableAssetEntry {
            equippable_group_id,
            catalog_address,
            metadata_uri,
            part_ids,
        } => assets.add_equippable_asset_entry(
            equippable_group_id,
            catalog_address,
            metadata_uri,
            part_ids,
        ),
        RMRKAction::AddAssetToToken {
            token_id,
            asset_id,
            replaces_asset_with_id,
        } => assets.add_asset_to_token(token_id, asset_id, replaces_asset_with_id),
        RMRKAction::AcceptAsset { token_id, asset_id } => assets.accept_asset(token_id, asset_id),
        RMRKAction::SetValidParentForEquippableGroup {
            equippable_group_id,
            slot_part_id,
            parent_id,
        } => assets.set_valid_parent_for_equippable_group(
            equippable_group_id,
            slot_part_id,
            parent_id,
        ),
        RMRKAction::CanTokenBeEquippedWithAssetIntoSlot {
            parent_id,
            token_id,
            asset_id,
            slot_part_id,
        } => assets.can_token_be_equipped_with_asset_into_slot(
            parent_id,
            token_id,
            asset_id,
            slot_part_id,
        ),
    }
}
#[no_mangle]
extern "C" fn state() {
    let rmrk = unsafe { RMRK.as_ref().expect("RMRK is not initialized") };
    let assets = unsafe { ASSETS.as_ref().expect("ASSETS is not initialized") };
    let mut rmrk_state: RMRKState = rmrk.into();
    let assets_state: AssetsState = assets.into();
    rmrk_state.assets = assets_state;
    msg::reply(rmrk_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to get the reply details");
    let tx_manager = unsafe { TX_MANAGER.as_mut().expect("Tx manager is not initialized") };
    let processing_msg_id = tx_manager
        .msg_sent_to_msg
        .remove(&reply_to)
        .expect("Receive reply on undefined message");
    let tx = tx_manager
        .txs
        .get_mut(&processing_msg_id)
        .expect("Message does not exist");
    let state = tx.state.clone();
    match state {
        TxState::CheckMsgSourceAccount(account) => {
            check_msg_source_account(tx, processing_msg_id, account)
        }
        TxState::MsgAddChildSent => check_received_reply(
            RMRKReply::PendingChildAdded,
            TxState::ReplyAddChildReceived,
            tx,
            processing_msg_id,
        ),
        TxState::MsgBurnChildSent => check_received_reply(
            RMRKReply::ChildBurnt,
            TxState::ReplyOnBurnChildReceived,
            tx,
            processing_msg_id,
        ),
        TxState::MsgTransferChildSent => check_received_reply(
            RMRKReply::ChildTransferred,
            TxState::ReplyOnTransferChildReceived,
            tx,
            processing_msg_id,
        ),
        TxState::MsgBurnFromParentSent => check_received_reply(
            RMRKReply::TokenBurnt,
            TxState::ReplyOnBurnFromParentReceived,
            tx,
            processing_msg_id,
        ),
        TxState::MsgCheckEquippableSent => check_received_reply_from_catalolg(
            CatalogReply::InEquippableList,
            TxState::ReplyCheckEquippableReceived,
            tx,
            processing_msg_id,
        ),
        TxState::MsgCanTokenBeEquippedSent => check_received_reply(
            RMRKReply::TokenBeEquippedWithAssetIntoSlot,
            TxState::ReplyCanTokenBeEquippedReceived,
            tx,
            processing_msg_id,
        ),
        TxState::MsgGetRootOwnerSent => get_root_owner(tx, processing_msg_id),
        _ => {}
    }
}

pub fn decode_root_owner(reply: Vec<u8>) -> ActorId {
    ActorId::decode(&mut &reply[..]).expect("Unable to decode ActorId")
}

pub fn get_root_owner_reply(tx: &mut Tx, processing_msg_id: MessageId) {
    let state = tx.state.clone();
    match state {
        TxState::MsgGetRootOwnerSent => {
            let reply: Result<RMRKReply, RMRKError> =
                msg::load().expect("Failed to decode the reply");
            match reply {
                Ok(RMRKReply::RootOwner(root_owner)) => {
                    tx.state = TxState::ReplyRootOwnerReceived;
                    tx.data = Some(root_owner.encode());
                }
                Ok(_) => {
                    tx.state = TxState::Error(RMRKError::UnexpectedReply);
                }
                Err(error) => {
                    tx.state = TxState::Error(error);
                }
            }
            exec::wake(processing_msg_id).expect("Failed to wake the message");
        }
        _ => {
            unreachable!()
        }
    }
}

fn check_approved_account(
    tx_manager: &mut TxManager,
    rmrk: &RMRKToken,
    token_id: TokenId,
) -> Result<(), RMRKError> {
    if let Some(approved_accounts) = rmrk.token_approvals.get(&token_id) {
        if approved_accounts.contains(&msg::source()) {
            tx_manager.set_tx_state(TxState::MsgSourceAccountChecked, MessageId::zero());
            return Ok(());
        }
    }

    let root_owner = rmrk.check_root_owner(tx_manager, token_id)?;
    if root_owner == msg::source() {
        tx_manager.set_tx_state(TxState::MsgSourceAccountChecked, MessageId::zero());
        tx_manager.set_tx_data(root_owner.encode());
        return Ok(());
    }

    Err(RMRKError::NotApprovedAccount)
}

fn check_root_owner(
    tx_manager: &mut TxManager,
    rmrk: &RMRKToken,
    token_id: TokenId,
) -> Result<(), RMRKError> {
    let root_owner = rmrk.check_root_owner(tx_manager, token_id)?;
    if root_owner == msg::source() {
        tx_manager.set_tx_state(TxState::MsgSourceAccountChecked, MessageId::zero());
        return Ok(());
    }
    Err(RMRKError::NotRootOwner)
}

fn check_received_reply(
    expected_reply: RMRKReply,
    next_state: TxState,
    tx: &mut Tx,
    processing_msg_id: MessageId,
) {
    let reply: Result<RMRKReply, RMRKError> = msg::load().expect("Failed to decode the reply");
    match reply {
        Ok(reply) => {
            if reply == expected_reply {
                tx.state = next_state;
            } else {
                tx.state = TxState::Error(RMRKError::UnexpectedReply);
            }
        }
        Err(error) => {
            tx.state = TxState::Error(error);
        }
    }
    exec::wake(processing_msg_id).expect("Failed to wake the message");
}

fn check_received_reply_from_catalolg(
    expected_reply: CatalogReply,
    next_state: TxState,
    tx: &mut Tx,
    processing_msg_id: MessageId,
) {
    let reply: Result<CatalogReply, CatalogError> =
        msg::load().expect("Failed to decode the reply");
    match reply {
        Ok(reply) => {
            if reply == expected_reply {
                tx.state = next_state;
            } else {
                tx.state = TxState::Error(RMRKError::UnexpectedReply);
            }
        }
        Err(_) => {
            tx.state = TxState::Error(RMRKError::ErrorInCatalog);
        }
    }
    exec::wake(processing_msg_id).expect("Failed to wake the message");
}

fn check_msg_source_account(tx: &mut Tx, processing_msg_id: MessageId, account: ActorId) {
    let reply: Result<RMRKReply, RMRKError> = msg::load().expect("Failed to decode the reply");
    match reply {
        Ok(RMRKReply::RootOwner(root_owner)) => {
            if root_owner == account {
                tx.state = TxState::MsgSourceAccountChecked;
                tx.data = Some(root_owner.encode());
            } else {
                tx.state = TxState::Error(RMRKError::NotRootOwner);
            }
        }
        Ok(_) => {
            tx.state = TxState::Error(RMRKError::UnexpectedReply);
        }
        Err(error) => {
            tx.state = TxState::Error(error);
        }
    }
    exec::wake(processing_msg_id).expect("Failed to wake the message");
}

fn get_root_owner(tx: &mut Tx, processing_msg_id: MessageId) {
    let reply: Result<RMRKReply, RMRKError> = msg::load().expect("Failed to decode the reply");
    match reply {
        Ok(RMRKReply::RootOwner(root_owner)) => {
            tx.data = Some(root_owner.encode());
            tx.state = TxState::ReplyRootOwnerReceived;
        }
        Ok(_) => {
            tx.state = TxState::Error(RMRKError::UnexpectedReply);
        }
        Err(error) => {
            tx.state = TxState::Error(error);
        }
    }
    exec::wake(processing_msg_id).expect("Failed to wake the message");
}
