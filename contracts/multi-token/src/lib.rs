#![no_std]
#![feature(const_btree_new)]

#[cfg(test)]
use codec::Encode;
use gstd::{debug, msg, prelude::*, ActorId};

pub mod base;
use base::{ERC1155TokenBase, ExtendERC1155TokenBase};

pub mod common;
use common::*;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug)]
struct ERC1155Token {
    name: String,
    symbol: String,
    base_uri: String,
    balances: BTreeMap<u128, BTreeMap<ActorId, u128>>,
    operator_approvals: BTreeMap<ActorId, BTreeMap<ActorId, bool>>,
}

static mut ERC1155_TOKEN: ERC1155Token = ERC1155Token {
    name: String::new(),
    symbol: String::new(),
    base_uri: String::new(),
    balances: BTreeMap::new(),
    operator_approvals: BTreeMap::new(),
};

impl ERC1155Token {
    fn get_balance(&self, account: &ActorId, id: &u128) -> u128 {
        *self
            .balances
            .get(id)
            .and_then(|m| m.get(account))
            .unwrap_or(&0)
    }

    fn set_balance(&mut self, account: &ActorId, id: &u128, amount: u128) {
        debug!(
            "before mint: {:?}, id: {:?}",
            self.balance_of(account, id),
            id
        );

        let mut _balance = self
            .balances
            .entry(*id)
            .or_default()
            .insert(*account, amount);

        debug!(
            "after mint: {:?}, id: {:?}",
            self.balance_of(account, id),
            id
        );
    }
}

impl ERC1155TokenBase for ERC1155Token {
    fn balance_of(&self, account: &ActorId, id: &u128) -> u128 {
        self.get_balance(account, id)
    }

    fn balance_of_batch(&self, accounts: &[ActorId], ids: &[u128]) -> Vec<BalanceOfBatchReply> {
        if accounts.len() != ids.len() {
            panic!("ERC1155: accounts and ids length mismatch")
        }

        let mut arr: Vec<BalanceOfBatchReply> = Vec::new();

        for (i, ele) in ids.iter().enumerate() {
            let account = accounts[i];
            let amount = self.balance_of(&account, ele);

            let obj = BalanceOfBatchReply {
                account,
                id: *ele,
                amount,
            };

            arr.push(obj);
        }

        arr
    }

    fn set_approval_for_all(&mut self, operator: &ActorId, approved: bool) {
        let owner = msg::source();

        if owner == *operator {
            panic!("ERC1155: setting approval status for self")
        }

        self.operator_approvals
            .entry(owner)
            .or_default()
            .insert(*operator, approved);
    }

    fn is_approved_for_all(&self, owner: &ActorId, operator: &ActorId) -> bool {
        self.operator_approvals.contains_key(owner)
            && *self.operator_approvals[owner]
                .get(operator)
                .unwrap_or(&false)
    }

    fn safe_transfer_from(&mut self, from: &ActorId, to: &ActorId, id: &u128, amount: u128) {
        if from == to {
            panic!("ERC1155: sender and recipient addresses are the same")
        }

        if !(from == &msg::source() || self.is_approved_for_all(from, &msg::source())) {
            panic!("ERC1155: caller is not owner nor approved")
        }

        if to == &ZERO_ID {
            panic!("ERC1155: transfer to the zero address")
        }

        let from_balance = self.balance_of(from, id);

        if from_balance < amount {
            panic!("ERC1155: insufficient balance for transfer")
        }

        self.set_balance(from, id, from_balance.saturating_sub(amount));
        let to_balance = self.balance_of(to, id);
        self.set_balance(to, id, to_balance.saturating_add(amount));
    }

    fn safe_batch_transfer_from(
        &mut self,
        from: &ActorId,
        to: &ActorId,
        ids: &[u128],
        amounts: &[u128],
    ) {
        if from == to {
            panic!("ERC1155: sender and recipient addresses are the same")
        }

        if !(from == &msg::source() || self.is_approved_for_all(from, &msg::source())) {
            panic!("ERC1155: caller is not owner nor approved")
        }

        if to == &ZERO_ID {
            panic!("ERC1155: transfer to the zero address")
        }

        if ids.len() != amounts.len() {
            panic!("ERC1155: ids and amounts length mismatch")
        }

        for (i, ele) in ids.iter().enumerate() {
            let amount = amounts[i];

            let from_balance = self.balance_of(from, ele);

            if from_balance < amount {
                panic!("ERC1155: insufficient balance for transfer")
            }

            self.set_balance(from, ele, from_balance.saturating_sub(amount));
            let to_balance = self.balance_of(to, ele);
            self.set_balance(to, ele, to_balance.saturating_add(amount));
        }
    }
}

impl ExtendERC1155TokenBase for ERC1155Token {
    fn owner_of(&self, id: &u128) -> bool {
        let owner = msg::source();

        self.balance_of(&owner, id) != 0
    }

    fn owner_of_batch(&self, ids: &[u128]) -> bool {
        for (_, ele) in ids.iter().enumerate() {
            let res = self.owner_of(ele);
            if !res {
                return false;
            }
        }

        true
    }

    fn mint(&mut self, account: &ActorId, id: &u128, amount: u128) {
        if account == &ZERO_ID {
            panic!("ERC1155: Mint to zero address")
        }
        let prev_balance = self.balance_of(account, id);
        self.set_balance(account, id, prev_balance.saturating_add(amount));
    }

    fn mint_batch(&mut self, account: &ActorId, ids: &[u128], amounts: &[u128]) {
        if account == &ZERO_ID {
            panic!("ERC1155: Mint to zero address")
        }

        if ids.len() != amounts.len() {
            panic!("ERC1155: ids and amounts length mismatch")
        }

        for (i, ele) in ids.iter().enumerate() {
            let amount = amounts[i];
            let prev_balance = self.balance_of(account, ele);
            self.set_balance(account, ele, prev_balance.saturating_add(amount));
        }
    }

    fn burn_batch(&mut self, ids: &[u128], amounts: &[u128]) {
        let owner = &msg::source();

        if ids.len() != amounts.len() {
            panic!("ERC1155: ids and amounts length mismatch")
        }

        if !self.owner_of_batch(ids) {
            panic!("ERC1155: have no ownership of ids")
        }

        for (i, ele) in ids.iter().enumerate() {
            let amount = amounts[i];

            let owner_balance = self.balance_of(owner, ele);

            if owner_balance < amount {
                panic!("ERC1155: burn amount exceeds balance")
            }

            self.set_balance(owner, ele, owner_balance.saturating_sub(amount));
        }
    }
}

gstd::metadata! {
    title: "ERC1155",
    init:
        input: InitConfig,
    handle:
        input: Action,
        output: Event,
    state:
        input: State,
        output: StateReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig");

    ERC1155_TOKEN.name = config.name;
    ERC1155_TOKEN.symbol = config.symbol;
    ERC1155_TOKEN.base_uri = config.base_uri;
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");

    let encoded = match query {
        State::Name => StateReply::Name(ERC1155_TOKEN.name.clone()).encode(),
        State::Symbol => StateReply::Name(ERC1155_TOKEN.symbol.clone()).encode(),
        State::Uri => StateReply::Uri(ERC1155_TOKEN.base_uri.clone()).encode(),
        State::BalanceOf(account, id) => {
            StateReply::Balance(ERC1155_TOKEN.balance_of(&account, &id)).encode()
        }
    };
    let result = gstd::macros::util::to_wasm_ptr(&(encoded[..]));
    core::mem::forget(encoded);
    result
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: Action = msg::load().expect("Could not load Action");
    match action {
        Action::Mint(account, id, amount) => {
            ERC1155_TOKEN.mint(&account, &id, amount);
            let transfer_data = TransferSingleReply {
                operator: msg::source(),
                from: ZERO_ID,
                to: account,
                id,
                amount,
            };

            msg::reply(Event::TransferSingle(transfer_data), 0).unwrap();
        }
        Action::BalanceOf(account, id) => {
            let balance = ERC1155_TOKEN.balance_of(&account, &id);
            msg::reply(Event::Balance(balance), 0).unwrap();
        }
        Action::BalanceOfBatch(accounts, ids) => {
            let res = ERC1155_TOKEN.balance_of_batch(&accounts, &ids);
            msg::reply(Event::BalanceOfBatch(res), 0).unwrap();
        }
        Action::MintBatch(account, ids, amounts) => {
            ERC1155_TOKEN.mint_batch(&account, &ids, &amounts);

            let payload = Event::TransferBatch {
                operator: msg::source(),
                from: ZERO_ID,
                to: account,
                ids,
                values: amounts,
            };
            msg::reply(payload, 0).unwrap();
        }

        Action::SafeTransferFrom(from, to, id, amount) => {
            ERC1155_TOKEN.safe_transfer_from(&from, &to, &id, amount);

            let transfer_data = TransferSingleReply {
                operator: msg::source(),
                from,
                to,
                id,
                amount,
            };

            msg::reply(Event::TransferSingle(transfer_data), 0).unwrap();
        }

        Action::SafeBatchTransferFrom(from, to, ids, amounts) => {
            ERC1155_TOKEN.safe_batch_transfer_from(&from, &to, &ids, &amounts);

            let payload = Event::TransferBatch {
                operator: msg::source(),
                from,
                to,
                ids,
                values: amounts,
            };

            msg::reply(payload, 0).unwrap();
        }

        Action::SetApprovalForAll(operator, approved) => {
            ERC1155_TOKEN.set_approval_for_all(&operator, approved);

            let owner = msg::source();

            let payload = Event::ApprovalForAll {
                owner,
                operator,
                approved,
            };

            msg::reply(payload, 0).unwrap();
        }

        Action::IsApprovedForAll(owner, operator) => {
            let approved = ERC1155_TOKEN.is_approved_for_all(&owner, &operator);

            let payload = Event::ApprovalForAll {
                owner,
                operator,
                approved,
            };

            msg::reply(payload, 0).unwrap();
        }

        Action::BurnBatch(ids, amounts) => {
            ERC1155_TOKEN.burn_batch(&ids, &amounts);

            let payload = Event::TransferBatch {
                operator: msg::source(),
                from: msg::source(),
                to: ZERO_ID,
                ids,
                values: amounts,
            };

            msg::reply(payload, 0).unwrap();
        }

        Action::OwnerOf(id) => {
            let res = ERC1155_TOKEN.owner_of(&id);
            msg::reply(res, 0).unwrap();
        }

        Action::OwnerOfBatch(ids) => {
            let res = ERC1155_TOKEN.owner_of_batch(&ids);

            msg::reply(res, 0).unwrap();
        }
    }
}
