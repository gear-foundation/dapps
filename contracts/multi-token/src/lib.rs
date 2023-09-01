#![no_std]

use gear_lib_derive::{MTKCore, MTKTokenState, StateKeeper};
use gear_lib_old::multitoken::{io::*, mtk_core::*, state::*};
use gstd::{
    collections::HashMap, errors::Result as GstdResult, msg, prelude::*, ActorId, MessageId,
};
use multi_token_io::*;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const NFT_COUNT: u128 = 1;

#[derive(Debug, Default, MTKTokenState, MTKCore, StateKeeper)]
pub struct SimpleMTK {
    #[MTKStateKeeper]
    pub tokens: MTKState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub supply: HashMap<TokenId, u128>,
}

pub trait SimpleMTKCore: MTKCore {
    fn mint(&mut self, account: ActorId, amount: u128, token_metadata: Option<TokenMetadata>);

    fn burn(&mut self, id: TokenId, amount: u128);

    fn transform(&mut self, id: TokenId, amount: u128, nfts: Vec<BurnToNFT>);
}

static mut CONTRACT: Option<SimpleMTK> = None;

#[no_mangle]
extern fn init() {
    let config: InitMTK = msg::load().expect("Unable to decode InitConfig");
    let mut multi_token = SimpleMTK::default();
    multi_token.tokens.name = config.name;
    multi_token.tokens.symbol = config.symbol;
    multi_token.tokens.base_uri = config.base_uri;
    multi_token.owner = msg::source();
    unsafe {
        CONTRACT = Some(multi_token);
    }
}

fn static_mut_state() -> &'static mut SimpleMTK {
    match unsafe { &mut CONTRACT } {
        Some(state) => state,
        None => unreachable!("State can't be uninitialized"),
    }
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

fn common_state() -> State {
    static_mut_state().into()
}

#[no_mangle]
extern fn state() {
    reply(common_state()).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

#[no_mangle]
unsafe extern fn handle() {
    let action: MyMTKAction = msg::load().expect("Could not load msg");
    let multi_token = CONTRACT.get_or_insert(SimpleMTK::default());
    match action {
        MyMTKAction::Mint {
            amount,
            token_metadata,
        } => SimpleMTKCore::mint(multi_token, msg::source(), amount, token_metadata),
        MyMTKAction::Burn { id, amount } => SimpleMTKCore::burn(multi_token, id, amount),
        MyMTKAction::BalanceOf { account, id } => {
            MTKCore::balance_of(multi_token, vec![account], vec![id])
        }
        MyMTKAction::BalanceOfBatch { accounts, ids } => {
            MTKCore::balance_of(multi_token, accounts, ids)
        }
        MyMTKAction::MintBatch {
            ids,
            amounts,
            tokens_metadata,
        } => MTKCore::mint(multi_token, &msg::source(), ids, amounts, tokens_metadata),
        MyMTKAction::TransferFrom {
            from,
            to,
            id,
            amount,
        } => MTKCore::transfer_from(multi_token, &from, &to, vec![id], vec![amount]),
        MyMTKAction::BatchTransferFrom {
            from,
            to,
            ids,
            amounts,
        } => MTKCore::transfer_from(multi_token, &from, &to, ids, amounts),
        MyMTKAction::BurnBatch { ids, amounts } => MTKCore::burn(multi_token, ids, amounts),
        MyMTKAction::Approve { account } => MTKCore::approve(multi_token, &account),
        MyMTKAction::RevokeApproval { account } => MTKCore::revoke_approval(multi_token, &account),
        MyMTKAction::Transform { id, amount, nfts } => {
            SimpleMTKCore::transform(multi_token, id, amount, nfts)
        }
    }
}

impl SimpleMTKCore for SimpleMTK {
    /// Mints a token.
    ///
    /// Arguments:
    /// * `account`: Which account to mint tokens to. Default - `msg::source()`,
    /// * `amount`: Token amount. In case of NFT - 1.
    /// * `token_metadata`: Token metadata, only applicable when minting an NFT. Otherwise - `None`.
    fn mint(&mut self, account: ActorId, amount: u128, token_metadata: Option<TokenMetadata>) {
        MTKCore::mint(
            self,
            &account,
            vec![(self.token_id)],
            vec![amount],
            vec![token_metadata],
        );
        self.supply.insert(self.token_id, amount);
        self.token_id = self.token_id.saturating_add(1);
    }

    /// Burns a token.
    ///
    /// Requirements:
    /// * sender MUST have sufficient amount of token.
    ///
    /// Arguments:
    /// * `id`: Token ID.
    /// * `amount`: Token's amount to be burnt.
    fn burn(&mut self, id: TokenId, amount: u128) {
        MTKCore::burn(self, vec![id], vec![amount]);
        let sup = self.supply(id);
        let mut _balance = self
            .supply
            .insert(self.token_id, sup.saturating_sub(amount));
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
    fn transform(&mut self, id: TokenId, amount: u128, nfts: Vec<BurnToNFT>) {
        // pre-checks
        let mut nft_count = 0;
        for info in &nfts {
            nft_count += info.nfts_ids.len();
        }
        if amount as usize != nft_count {
            panic!("MTK: amount of burnt tokens MUST be equal to the amount of nfts.");
        }

        // burn FT (not to produce another message - just simply use burn_impl)
        self.assert_can_burn(&msg::source(), &id, amount);
        self.burn_impl(&id, amount);

        let sup = self.supply(id);
        let mut _balance = self
            .supply
            .insert(self.token_id, sup.saturating_sub(amount));
        let mut ids = vec![];

        for burn_info in nfts {
            if burn_info.account.is_zero() {
                panic!("MTK: Mint to zero address");
            }
            if burn_info.nfts_ids.len() != burn_info.nfts_metadata.len() {
                panic!("MTK: ids and amounts length mismatch");
            }
            burn_info
                .nfts_metadata
                .into_iter()
                .enumerate()
                .for_each(|(i, meta)| {
                    self.mint_impl(&burn_info.account, &burn_info.nfts_ids[i], NFT_COUNT, meta)
                });
            for id in burn_info.nfts_ids {
                ids.push(id);
            }
        }

        msg::reply(
            MTKEvent::Transfer {
                operator: msg::source(),
                from: ActorId::zero(),
                to: ActorId::zero(),
                ids: ids.to_vec(),
                amounts: vec![NFT_COUNT; amount as usize],
            },
            0,
        )
        .expect("Error during a reply with MTKEvent::Transfer");
    }
}

impl From<&mut SimpleMTK> for State {
    fn from(value: &mut SimpleMTK) -> Self {
        let balances = value
            .tokens
            .balances
            .iter()
            .map(|(k, v)| (*k, v.iter().map(|(a, b)| (*a, *b)).collect()))
            .collect();
        let approvals = value
            .tokens
            .approvals
            .iter()
            .map(|(k, v)| (*k, v.iter().map(|(a, b)| (*a, *b)).collect()))
            .collect();
        let token_metadata = value
            .tokens
            .token_metadata
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
        let owners = value.tokens.owners.iter().map(|(k, v)| (*k, *v)).collect();
        let supply = value.supply.iter().map(|(k, v)| (*k, *v)).collect();
        Self {
            name: value.tokens.name.clone(),
            symbol: value.tokens.symbol.clone(),
            base_uri: value.tokens.base_uri.clone(),
            balances,
            approvals,
            token_metadata,
            owners,
            token_id: value.token_id,
            owner: value.owner,
            supply,
        }
    }
}
