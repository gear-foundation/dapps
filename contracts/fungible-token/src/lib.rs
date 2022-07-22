#![no_std]

#[cfg(test)]
mod tests;

use ft_io::*;
use gstd::{exec, msg, prelude::*, ActorId};

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug, Default)]
struct FungibleToken {
    /// Name of the token.
    name: String,
    /// Symbol of the token.
    symbol: String,
    /// Total supply of the token.
    total_supply: u128,
    /// Map to hold balances of token holders.
    balances: BTreeMap<ActorId, u128>,
    /// Map to hold allowance information of token holders.
    allowances: BTreeMap<ActorId, BTreeMap<ActorId, u128>>,
    /// Token's decimals.
    pub decimals: u8,
}

static mut FUNGIBLE_TOKEN: Option<FungibleToken> = None;

impl FungibleToken {
    /// Executed on receiving `fungible-token-messages::MintInput`.
    fn mint(&mut self, amount: u128) {
        self.balances
            .entry(msg::source())
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);
        self.total_supply += amount;
        msg::reply(
            FTEvent::Transfer {
                from: ZERO_ID,
                to: msg::source(),
                amount,
            },
            0,
        )
        .unwrap();
    }
    /// Executed on receiving `fungible-token-messages::BurnInput`.
    fn burn(&mut self, amount: u128) {
        if self.balances.get(&msg::source()).unwrap_or(&0) < &amount {
            panic!("Amount exceeds account balance");
        }
        self.balances
            .entry(msg::source())
            .and_modify(|balance| *balance -= amount);
        self.total_supply -= amount;

        msg::reply(
            FTEvent::Transfer {
                from: msg::source(),
                to: ZERO_ID,
                amount,
            },
            0,
        )
        .unwrap();
    }
    /// Executed on receiving `fungible-token-messages::TransferInput` or `fungible-token-messages::TransferFromInput`.
    /// Transfers `amount` tokens from `sender` account to `recipient` account.
    fn transfer(&mut self, from: &ActorId, to: &ActorId, amount: u128) {
        if from == &ZERO_ID || to == &ZERO_ID {
            panic!("Zero addresses");
        };
        if !self.can_transfer(from, amount) {
            panic!("Not allowed to transfer")
        }
        if self.balances.get(from).unwrap_or(&0) < &amount {
            panic!("Amount exceeds account balance");
        }
        self.balances
            .entry(*from)
            .and_modify(|balance| *balance -= amount);
        self.balances
            .entry(*to)
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);
        msg::reply(
            FTEvent::Transfer {
                from: *from,
                to: *to,
                amount,
            },
            0,
        )
        .unwrap();
    }

    /// Executed on receiving `fungible-token-messages::ApproveInput`.
    fn approve(&mut self, to: &ActorId, amount: u128) {
        if to == &ZERO_ID {
            panic!("Approve to zero address");
        }
        self.allowances
            .entry(msg::source())
            .or_default()
            .insert(*to, amount);
        msg::reply(
            FTEvent::Approve {
                from: msg::source(),
                to: *to,
                amount,
            },
            0,
        )
        .unwrap();
    }

    fn can_transfer(&mut self, from: &ActorId, amount: u128) -> bool {
        if from == &msg::source()
            || from == &exec::origin()
            || self.balances.get(&msg::source()).unwrap_or(&0) >= &amount
        {
            return true;
        }
        if let Some(allowed_amount) = self
            .allowances
            .get(from)
            .and_then(|m| m.get(&msg::source()))
        {
            if allowed_amount >= &amount {
                self.allowances.entry(*from).and_modify(|m| {
                    m.entry(msg::source()).and_modify(|a| *a -= amount);
                });
                return true;
            }
        }
        false
    }
}

gstd::metadata! {
    title: "FungibleToken",
    init:
        input: InitConfig,
    handle:
        input: FTAction,
        output: FTEvent,
    state:
        input: State,
        output: StateReply,
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: FTAction = msg::load().expect("Could not load Action");
    let ft: &mut FungibleToken = FUNGIBLE_TOKEN.get_or_insert(FungibleToken::default());
    match action {
        FTAction::Mint(amount) => {
            ft.mint(amount);
        }
        FTAction::Burn(amount) => {
            ft.burn(amount);
        }
        FTAction::Transfer { from, to, amount } => {
            ft.transfer(&from, &to, amount);
        }
        FTAction::Approve { to, amount } => {
            ft.approve(&to, amount);
        }
        FTAction::TotalSupply => {
            msg::reply(FTEvent::TotalSupply(ft.total_supply), 0).unwrap();
        }
        FTAction::BalanceOf(account) => {
            let balance = ft.balances.get(&account).unwrap_or(&0);
            msg::reply(FTEvent::Balance(*balance), 0).unwrap();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig");
    let ft = FungibleToken {
        name: config.name,
        symbol: config.symbol,
        decimals: config.decimals,
        ..FungibleToken::default()
    };
    FUNGIBLE_TOKEN = Some(ft);
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");
    let ft: &mut FungibleToken = FUNGIBLE_TOKEN.get_or_insert(FungibleToken::default());
    let encoded = match query {
        State::Name => StateReply::Name(ft.name.clone()).encode(),
        State::Symbol => StateReply::Name(ft.symbol.clone()).encode(),
        State::Decimals => StateReply::Decimals(ft.decimals).encode(),
        State::TotalSupply => StateReply::TotalSupply(ft.total_supply).encode(),
        State::BalanceOf(account) => {
            let balance = ft.balances.get(&account).unwrap_or(&0);
            StateReply::Balance(*balance).encode()
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}

#[no_mangle]
pub unsafe extern "C" fn handle_reply() {}
