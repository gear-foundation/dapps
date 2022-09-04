#![no_std]
use gear_lib::multitoken::{io::*, mtk_core::*, state::*};
use gear_lib_derive::{MTKCore, MTKTokenState, StateKeeper};
use gstd::{msg, prelude::*, ActorId};
use multitoken_io::*;

#[derive(Debug, Default, MTKTokenState, MTKCore, StateKeeper)]
pub struct SimpleMTK {
    #[MTKStateKeeper]
    pub tokens: MTKState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub supply: BTreeMap<TokenId, u128>,
}

pub trait SimpleMTKCore: MTKCore {
    fn mint(&mut self, account: ActorId, amount: u128, token_metadata: Option<TokenMetadata>);

    fn burn(&mut self, id: TokenId, amount: u128);
}

static mut CONTRACT: Option<SimpleMTK> = None;

gstd::metadata! {
    title: "MTK",
    init:
        input: InitMTK,
    handle:
        input: MyMTKAction,
        output: Vec<u8>,
    state:
        input: MTKQuery,
        output: MTKQueryReply,
}

#[no_mangle]
extern "C" fn init() {
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

#[no_mangle]
unsafe extern "C" fn handle() {
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
    }
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: MTKQuery = msg::load().expect("failed to decode input argument");
    let multi_token = CONTRACT.get_or_insert(SimpleMTK::default());
    let encoded = MTKTokenState::proc_state(multi_token, query).expect("error");
    gstd::util::to_leak_ptr(encoded)
}

impl SimpleMTKCore for SimpleMTK {
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

    fn burn(&mut self, id: TokenId, amount: u128) {
        MTKCore::burn(self, vec![id], vec![amount]);
        let sup = self.supply(id);
        let mut _balance = self
            .supply
            .insert(self.token_id, sup.saturating_sub(amount));
    }
}
