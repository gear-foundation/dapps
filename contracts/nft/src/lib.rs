#![no_std]

use gstd::{
    collections::{HashMap, HashSet},
    msg,
    prelude::*,
    ActorId,
};
use nft_io::*;

#[derive(Debug, Default)]
pub struct Nft {
    pub owner_by_id: HashMap<TokenId, ActorId>,
    pub token_approvals: HashMap<TokenId, ActorId>,
    pub token_metadata_by_id: HashMap<TokenId, TokenMetadata>,
    pub tokens_for_owner: HashMap<ActorId, HashSet<TokenId>>,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub collection: Collection,
    pub config: Config,
}

static mut NFT: Option<Nft> = None;

#[no_mangle]
unsafe extern fn init() {
    let init: InitNft = msg::load().expect("Unable to decode InitNft");

    let nft = Nft {
        collection: init.collection,
        config: init.config,
        owner: msg::source(),
        ..Default::default()
    };
    NFT = Some(nft);
}

impl Nft {
    /// Mint a new nft using `TokenMetadata`
    fn mint(&mut self, to: &ActorId, token_metadata: TokenMetadata) -> NftEvent {
        self.check_config();
        self.check_zero_address(to);
        self.owner_by_id.insert(self.token_id, *to);
        self.tokens_for_owner
            .entry(*to)
            .and_modify(|tokens| {
                tokens.insert(self.token_id);
            })
            .or_insert_with(|| HashSet::from([self.token_id]));
        self.token_metadata_by_id
            .insert(self.token_id, token_metadata.clone());

        self.token_id += 1;

        NftEvent::Minted {
            to: *to,
            token_metadata,
        }
    }
    /// Burn nft by `TokenId`
    fn burn(&mut self, token_id: TokenId) -> NftEvent {
        let owner = *self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");

        self.check_owner(&owner);
        self.owner_by_id.remove(&token_id);
        self.token_metadata_by_id.remove(&token_id);

        if let Some(tokens) = self.tokens_for_owner.get_mut(&owner) {
            tokens.remove(&token_id);
            if tokens.is_empty() {
                self.tokens_for_owner.remove(&owner);
            }
        }
        self.token_approvals.remove(&token_id);

        NftEvent::Burnt { token_id }
    }
    ///  Transfer token from `token_id` to address `to`
    fn transfer(&mut self, to: &ActorId, token_id: TokenId) -> NftEvent {
        let owner = *self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");

        self.can_transfer(token_id, &owner);
        self.check_zero_address(to);
        // assign new owner
        self.owner_by_id
            .entry(token_id)
            .and_modify(|owner| *owner = *to);
        // push token to new owner
        self.tokens_for_owner
            .entry(*to)
            .and_modify(|tokens| {
                tokens.insert(token_id);
            })
            .or_insert_with(|| HashSet::from([token_id]));
        // remove token from old owner
        if let Some(tokens) = self.tokens_for_owner.get_mut(&owner) {
            tokens.remove(&token_id);
            if tokens.is_empty() {
                self.tokens_for_owner.remove(&owner);
            }
        }
        // remove approvals if any
        self.token_approvals.remove(&token_id);

        NftEvent::Transferred {
            from: owner,
            to: *to,
            token_id,
        }
    }
    ///  Approve token from `token_id` to address `to`
    fn approve(&mut self, to: &ActorId, token_id: TokenId) -> NftEvent {
        let owner = self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.check_owner(owner);
        self.check_zero_address(to);
        self.check_approve(&token_id);
        self.token_approvals.insert(token_id, *to);

        NftEvent::Approved {
            owner: *owner,
            approved_account: *to,
            token_id,
        }
    }
    /// Get `ActorId` of the nft owner with `token_id`
    fn owner(&self, token_id: TokenId) -> NftEvent {
        let owner = self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");

        NftEvent::Owner {
            owner: *owner,
            token_id,
        }
    }
    /// Get confirmation about approval to address `to` and `token_id`
    fn is_approved_to(&self, to: &ActorId, token_id: TokenId) -> NftEvent {
        if !self.owner_by_id.contains_key(&token_id) {
            panic!("Token does not exist")
        }
        self.token_approvals.get(&token_id).map_or_else(
            || NftEvent::CheckIfApproved {
                to: *to,
                token_id,
                approved: false,
            },
            |approval_id| NftEvent::CheckIfApproved {
                to: *to,
                token_id,
                approved: *approval_id == *to,
            },
        )
    }

    /// Checking the configuration with current contract data
    fn check_config(&self) {
        if let Some(max_mint_count) = self.config.max_mint_count {
            if max_mint_count <= self.token_metadata_by_id.len() as u128 {
                panic!(
                    "Mint impossible because max minting count {} limit exceeded",
                    max_mint_count
                );
            }
        }
    }
    /// Check for ZERO_ID address
    fn check_zero_address(&self, account: &ActorId) {
        if account == &ZERO_ID {
            panic!("NonFungibleToken: zero address");
        }
    }
    /// Checks that `msg::source()` is the owner of the token with indicated `token_id`
    fn check_owner(&self, owner: &ActorId) {
        if owner != &msg::source() {
            panic!("NonFungibleToken: access denied");
        }
    }
    /// Checks that `msg::source()` is allowed to manage the token with indicated `token_id`
    fn can_transfer(&self, token_id: TokenId, owner: &ActorId) {
        if let Some(approved_accounts) = self.token_approvals.get(&token_id) {
            if approved_accounts == &msg::source() {
                return;
            }
        }
        self.check_owner(owner);
    }
    /// Check the existence of a approve
    fn check_approve(&self, token_id: &TokenId) {
        if self.token_approvals.contains_key(token_id) {
            panic!("Approve has already been issued");
        }
    }
}

#[no_mangle]
extern fn handle() {
    let action: NftAction = msg::load().expect("Could not load NftAction");
    let nft = unsafe { NFT.as_mut().expect("`NFT` is not initialized.") };
    let result = match action {
        NftAction::Mint { to, token_metadata } => nft.mint(&to, token_metadata),
        NftAction::Burn { token_id } => nft.burn(token_id),
        NftAction::Transfer { to, token_id } => nft.transfer(&to, token_id),
        NftAction::Approve { to, token_id } => nft.approve(&to, token_id),
        NftAction::GetOwner { token_id } => nft.owner(token_id),
        NftAction::CheckIfApproved { to, token_id } => nft.is_approved_to(&to, token_id),
    };
    msg::reply(result, 0).expect("Failed to encode or reply with `NftEvent`.");
}

#[no_mangle]
extern fn state() {
    let nft = unsafe { NFT.take().expect("Unexpected error in taking state") };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    match query {
        StateQuery::All => {
            msg::reply(StateReply::All(nft.into()), 0).expect("Unable to share the state");
        }
        StateQuery::Config => {
            msg::reply(StateReply::Config(nft.config), 0).expect("Unable to share the state");
        }
        StateQuery::Collection => {
            msg::reply(StateReply::Collection(nft.collection), 0)
                .expect("Unable to share the state");
        }
        StateQuery::Owner => {
            msg::reply(StateReply::Owner(nft.owner), 0).expect("Unable to share the state");
        }
        StateQuery::CurrentTokenId => {
            msg::reply(StateReply::CurrentTokenId(nft.token_id), 0)
                .expect("Unable to share the state");
        }
        StateQuery::OwnerById { token_id } => {
            msg::reply(
                StateReply::OwnerById(nft.owner_by_id.get(&token_id).cloned()),
                0,
            )
            .expect("Unable to share the state");
        }
        StateQuery::TokenApprovals { token_id } => {
            let approval = nft.token_approvals.get(&token_id).cloned();
            msg::reply(StateReply::TokenApprovals(approval), 0).expect("Unable to share the state");
        }
        StateQuery::TokenMetadata { token_id } => {
            msg::reply(
                StateReply::TokenMetadata(nft.token_metadata_by_id.get(&token_id).cloned()),
                0,
            )
            .expect("Unable to share the state");
        }
        StateQuery::OwnerTokens { owner } => {
            let tokens = nft
                .tokens_for_owner
                .get(&owner)
                .map(|hashset| hashset.iter().cloned().collect());
            msg::reply(StateReply::OwnerTokens(tokens), 0).expect("Unable to share the state");
        }
    }
}

impl From<Nft> for State {
    fn from(value: Nft) -> Self {
        let Nft {
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            token_id,
            owner,
            collection,
            config,
        } = value;

        let owner_by_id = owner_by_id.into_iter().collect();

        let token_approvals = token_approvals.into_iter().collect();

        let token_metadata_by_id = token_metadata_by_id.into_iter().collect();

        let tokens_for_owner = tokens_for_owner
            .into_iter()
            .map(|(id, tokens)| (id, tokens.into_iter().collect()))
            .collect();

        Self {
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            token_id,
            owner,
            collection,
            config,
        }
    }
}
