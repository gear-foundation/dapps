#![no_std]
use gstd::{
    collections::{HashMap, HashSet},
    msg,
    prelude::*,
    ActorId,
};
use multi_token_io::*;

const NFT_COUNT: u128 = 1;

#[derive(Debug, Default)]
pub struct SimpleMtk {
    pub tokens: MtkData,
    pub creator: ActorId,
    pub supply: HashMap<TokenId, u128>,
}

static mut CONTRACT: Option<SimpleMtk> = None;

#[no_mangle]
extern fn init() {
    let InitMtk {
        name,
        symbol,
        base_uri,
    } = msg::load().expect("Unable to decode `InitMtk`");

    unsafe {
        CONTRACT = Some(SimpleMtk {
            tokens: MtkData {
                name,
                symbol,
                base_uri,
                ..Default::default()
            },
            creator: msg::source(),
            ..Default::default()
        });
    }
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

#[no_mangle]
extern fn handle() {
    let action: MtkAction = msg::load().expect("Failed to decode `MtkAction` message.");
    let multi_token = unsafe { CONTRACT.as_mut().expect("`SimpleMtk` is not initialized.") };

    let reply = match action {
        MtkAction::Mint {
            id,
            amount,
            token_metadata,
        } => multi_token.mint(&msg::source(), vec![id], vec![amount], vec![token_metadata]),
        MtkAction::Burn { id, amount } => multi_token.burn(vec![id], vec![amount]),
        MtkAction::BalanceOf { account, id } => multi_token.balance_of(vec![account], vec![id]),
        MtkAction::BalanceOfBatch { accounts, ids } => multi_token.balance_of(accounts, ids),
        MtkAction::MintBatch {
            ids,
            amounts,
            tokens_metadata,
        } => multi_token.mint(&msg::source(), ids, amounts, tokens_metadata),
        MtkAction::TransferFrom {
            from,
            to,
            id,
            amount,
        } => multi_token.transfer_from(&from, &to, vec![id], vec![amount]),
        MtkAction::BatchTransferFrom {
            from,
            to,
            ids,
            amounts,
        } => multi_token.transfer_from(&from, &to, ids, amounts),
        MtkAction::BurnBatch { ids, amounts } => multi_token.burn(ids, amounts),
        MtkAction::Approve { account } => multi_token.approve(&account),
        MtkAction::RevokeApproval { account } => multi_token.revoke_approval(&account),
        MtkAction::Transform { id, amount, nfts } => multi_token.transform(id, amount, nfts),
    };
    msg::reply(reply, 0).expect("Failed to encode or reply with `Result<MtkEvent, MtkError>`.");
}

impl SimpleMtk {
    /// Mints a token.
    ///
    /// Arguments:
    /// * `account`: Which account to mint tokens to. Default - `msg::source()`,
    /// * `ids`: The vector of token IDs, must be unique
    /// * `amount`: The vector of token amounts. In case of NFT - 1.
    /// * `token_metadata`: Token metadata, only applicable when minting an NFT. Otherwise - `None`.
    fn mint(
        &mut self,
        account: &ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
        meta: Vec<Option<TokenMetadata>>,
    ) -> Result<MtkEvent, MtkError> {
        if *account == ActorId::zero() {
            return Err(MtkError::ZeroAddress);
        }

        if ids.len() != amounts.len() || ids.len() != meta.len() {
            return Err(MtkError::LengthMismatch);
        }

        let unique_ids: HashSet<_> = ids.clone().into_iter().collect();

        if ids.len() != unique_ids.len() {
            return Err(MtkError::IdIsNotUnique);
        }

        ids.iter().enumerate().try_for_each(|(i, id)| {
            if self.tokens.token_metadata.contains_key(id) {
                return Err(MtkError::TokenAlreadyExists);
            } else if let Some(_token_meta) = &meta[i] {
                if amounts[i] > 1 {
                    return Err(MtkError::MintMetadataToFungibleToken);
                }
            }
            Ok(())
        })?;

        for (i, meta_item) in meta.into_iter().enumerate() {
            self.mint_impl(account, &ids[i], amounts[i], meta_item)?;
        }
        for (id, amount) in ids.iter().zip(amounts.iter()) {
            self.supply
                .entry(*id)
                .and_modify(|quantity| {
                    *quantity = quantity.saturating_add(*amount);
                })
                .or_insert(*amount);
        }

        Ok(MtkEvent::Transfer {
            from: ActorId::zero(),
            to: *account,
            ids,
            amounts,
        })
    }

    fn mint_impl(
        &mut self,
        account: &ActorId,
        id: &TokenId,
        amount: u128,
        meta: Option<TokenMetadata>,
    ) -> Result<(), MtkError> {
        if let Some(metadata) = meta {
            self.tokens.token_metadata.insert(*id, metadata);
            // since we have metadata = means we have an nft, so add it to the owners
            self.tokens.owners.insert(*id, *account);
        }
        let prev_balance = self.get_balance(account, id);
        self.set_balance(account, id, prev_balance.saturating_add(amount));
        Ok(())
    }

    /// Burns a token.
    ///
    /// Requirements:
    /// * sender MUST have sufficient amount of token.
    ///
    /// Arguments:
    /// * `ids`: The vector of token IDs
    /// * `amounts`: The vector of token amounts to be burnt.

    fn burn(&mut self, ids: Vec<TokenId>, amounts: Vec<u128>) -> Result<MtkEvent, MtkError> {
        if ids.len() != amounts.len() {
            return Err(MtkError::LengthMismatch);
        }

        let msg_src = &msg::source();
        ids.iter()
            .zip(amounts.clone())
            .try_for_each(|(id, amount)| {
                if self.tokens.token_metadata.contains_key(id) && amount > 1 {
                    return Err(MtkError::AmountGreaterThanOneForNft);
                }
                self.check_opportunity_burn(msg_src, id, amount)
            })?;

        ids.iter()
            .enumerate()
            .for_each(|(i, id)| self.burn_impl(msg_src, id, amounts[i]));

        for (id, amount) in ids.iter().zip(amounts.iter()) {
            let quantity = self.supply.get_mut(id).ok_or(MtkError::WrongId)?;
            *quantity = quantity.saturating_sub(*amount);
        }

        Ok(MtkEvent::Transfer {
            from: *msg_src,
            to: ActorId::zero(),
            ids,
            amounts,
        })
    }

    fn burn_impl(&mut self, msg_source: &ActorId, id: &TokenId, amount: u128) {
        self.tokens.owners.remove(id);
        self.set_balance(
            msg_source,
            id,
            self.get_balance(msg_source, id).saturating_sub(amount),
        );
    }

    /// Returns the amount of multiple specific tokens multiple users have
    /// (in case all input length is 1 - simple balance_of)
    /// Arguments:
    /// * `accounts`: The vectors of IDs of the actor
    /// * `id`: The vector of token IDs which balance will be returned
    fn balance_of(&self, accounts: Vec<ActorId>, ids: Vec<TokenId>) -> Result<MtkEvent, MtkError> {
        if accounts.len() != ids.len() {
            return Err(MtkError::LengthMismatch);
        }

        let res = ids
            .iter()
            .zip(accounts)
            .map(|(id, account)| BalanceReply {
                account,
                id: *id,
                amount: self.get_balance(&account, id),
            })
            .collect();

        Ok(MtkEvent::BalanceOf(res))
    }

    /// Transfers multiple tokens to a new user (in case all input length is 1 - simple transfer)
    /// Requirements:
    /// * Only the token owner or approved account can call that action
    /// * `to` must be a non-zero account
    /// * `ids` element must be the ID of the existing token
    /// * `amounts` element must not exceed from's balance
    ///   Arguments:
    /// * `from`: An account from which token will be transferred
    /// * `to`: An account to which token will be transferred
    /// * `ids`: The vector of IDs of transferred token
    /// * `amounts`: The vector of amounts of transferred token
    fn transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
    ) -> Result<MtkEvent, MtkError> {
        let msg_src = msg::source();
        if from == to {
            return Err(MtkError::SenderAndRecipientAddressesAreSame);
        }

        if from != &msg_src && !self.is_approved(from, &msg_src) {
            return Err(MtkError::CallerIsNotOwnerOrApproved);
        }

        if to == &ActorId::zero() {
            return Err(MtkError::ZeroAddress);
        }

        if ids.len() != amounts.len() {
            return Err(MtkError::LengthMismatch);
        }

        for (id, amount) in ids.iter().zip(amounts.clone()) {
            self.check_opportunity_transfer(from, id, amount)?;
        }

        for (i, id) in ids.iter().enumerate() {
            self.transfer_from_impl(from, to, id, amounts[i])?;
        }

        Ok(MtkEvent::Transfer {
            from: *from,
            to: *to,
            ids,
            amounts,
        })
    }

    fn transfer_from_impl(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        id: &TokenId,
        amount: u128,
    ) -> Result<(), MtkError> {
        let from_balance = self.get_balance(from, id);
        self.set_balance(from, id, from_balance.saturating_sub(amount));
        let to_balance = self.get_balance(to, id);
        self.set_balance(to, id, to_balance.saturating_add(amount));
        Ok(())
    }

    /// Gives a right to another account to manage its tokens
    /// Requirements:
    /// * Only the token owner can call that action
    /// * `to` must be a non-zero account
    ///   Arguments:
    /// * `to`: An account that will be approved to manage the tokens
    fn approve(&mut self, to: &ActorId) -> Result<MtkEvent, MtkError> {
        if to == &ActorId::zero() {
            return Err(MtkError::ZeroAddress);
        }
        let msg_src = &msg::source();
        self.tokens
            .approvals
            .entry(*msg_src)
            .and_modify(|approvals| {
                approvals.insert(*to);
            })
            .or_insert_with(|| HashSet::from([*to]));

        Ok(MtkEvent::Approval {
            from: *msg_src,
            to: *to,
        })
    }

    /// Removed a right to another account to manage its tokens
    /// Requirements:
    /// * Only the token owner can call that action
    /// * `to` must be a non-zero account
    ///   Arguments:
    /// * `to`: An account that won't be able to manage the tokens
    fn revoke_approval(&mut self, to: &ActorId) -> Result<MtkEvent, MtkError> {
        let msg_src = &msg::source();

        let approvals = self
            .tokens
            .approvals
            .get_mut(msg_src)
            .ok_or(MtkError::NoApprovals)?;
        if !approvals.remove(to) {
            return Err(MtkError::ThereIsNoThisApproval);
        }

        Ok(MtkEvent::RevokeApproval {
            from: *msg_src,
            to: *to,
        })
    }

    /// Transforms FT tokens to multiple NFTs.
    ///
    /// Requirements:
    /// * a sender MUST have sufficient amount of tokens to burn,
    /// * a sender MUST be the owner.
    ///
    /// Arguments:
    /// * `id`: Token ID.
    /// * `amount`: Token's amount to be burnt.
    /// * `accounts`: To which accounts to mint NFT.
    /// * `nft_ids`: NFTs' IDs to be minted.
    /// * `nfts_metadata`: NFT's metadata.
    fn transform(
        &mut self,
        id: TokenId,
        amount: u128,
        nfts: Vec<BurnToNFT>,
    ) -> Result<MtkEvent, MtkError> {
        // pre-checks
        let mut nft_count = 0;
        for info in &nfts {
            nft_count += info.nfts_ids.len();
        }
        if amount as usize != nft_count {
            return Err(MtkError::IncorrectData);
        }

        // burn FT (not to produce another message - just simply use burn_impl)
        let msg_src = &msg::source();
        self.check_opportunity_burn(msg_src, &id, amount)?;
        self.burn_impl(msg_src, &id, amount);

        for burn_info in nfts.iter() {
            if burn_info.account.is_zero() {
                return Err(MtkError::ZeroAddress);
            }
            if burn_info.nfts_ids.len() != burn_info.nfts_metadata.len() {
                return Err(MtkError::LengthMismatch);
            }
        }

        let mut ids = vec![];
        for burn_info in nfts {
            burn_info
                .nfts_metadata
                .into_iter()
                .zip(burn_info.nfts_ids.iter())
                .try_for_each(|(meta, &id)| {
                    self.mint_impl(&burn_info.account, &id, NFT_COUNT, meta)
                })?;

            ids.extend_from_slice(&burn_info.nfts_ids);
        }

        let quantity = self.supply.get_mut(&id).ok_or(MtkError::WrongId)?;
        *quantity = quantity.saturating_sub(amount);

        Ok(MtkEvent::Transfer {
            from: ActorId::zero(),
            to: ActorId::zero(),
            ids,
            amounts: vec![NFT_COUNT; amount as usize],
        })
    }

    fn get_balance(&self, account: &ActorId, id: &TokenId) -> u128 {
        *self
            .tokens
            .balances
            .get(id)
            .and_then(|m| m.get(account))
            .unwrap_or(&0)
    }

    fn set_balance(&mut self, account: &ActorId, id: &TokenId, amount: u128) {
        self.tokens
            .balances
            .entry(*id)
            .or_default()
            .insert(*account, amount);
    }

    fn check_opportunity_burn(
        &mut self,
        owner: &ActorId,
        id: &TokenId,
        amount: u128,
    ) -> Result<(), MtkError> {
        if self.get_balance(owner, id) < amount {
            return Err(MtkError::NotEnoughBalance);
        }
        Ok(())
    }

    fn check_opportunity_transfer(
        &self,
        from: &ActorId,
        id: &u128,
        amount: u128,
    ) -> Result<(), MtkError> {
        if self.get_balance(from, id) < amount {
            return Err(MtkError::InsufficientBalanceForTransfer);
        }
        Ok(())
    }
    fn is_approved(&self, owner: &ActorId, msg_source: &ActorId) -> bool {
        if let Some(approvals) = self.tokens.approvals.get(owner) {
            return approvals.contains(msg_source);
        }
        false
    }
}

impl From<SimpleMtk> for State {
    fn from(value: SimpleMtk) -> Self {
        let SimpleMtk {
            tokens,
            creator,
            supply,
        } = value;

        let MtkData {
            name,
            symbol,
            base_uri,
            balances,
            approvals,
            token_metadata,
            owners,
        } = tokens;

        let balances = balances
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let approvals = approvals
            .into_iter()
            .map(|(k, v)| (k, v.iter().copied().collect()))
            .collect();
        let token_metadata = token_metadata.into_iter().collect();
        let owners = owners.into_iter().collect();
        let supply = supply.into_iter().collect();
        Self {
            name,
            symbol,
            base_uri,
            balances,
            approvals,
            token_metadata,
            owners,
            creator,
            supply,
        }
    }
}
