use crate::fungible_token::{io::*, state::*};
use gstd::{msg, prelude::*, ActorId};

const ZERO_ID: ActorId = ActorId::zero();

pub trait FTCore: FTStateKeeper {
    /// Mints `amount` of token
    ///
    /// Requirements: None
    /// Arguments:
    /// * `amount`: The amount of token to be minted (actually have no limit)
    fn mint(&mut self, to: &ActorId, amount: u128) {
        self.get_mut()
            .balances
            .entry(*to)
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);
        self.get_mut().total_supply += amount;
        msg::reply(
            FTTransfer {
                from: ZERO_ID,
                to: *to,
                amount,
            }
            .encode(),
            0,
        )
        .expect("Error during a reply with FTEvent::FTTransfer");
    }

    /// Burns `amount` of token
    ///
    /// Requirements:
    /// * `msg::source()` MUST have enough tokens on his balance
    /// Arguments:
    /// `amount`: The amount of token to be burnt
    fn burn(&mut self, amount: u128) {
        if self
            .get()
            .balances
            .get(&msg::source())
            .expect("The account has no balance at all")
            < &amount
        {
            panic!("Amount exceeds account's balance");
        }
        self.get_mut()
            .balances
            .entry(msg::source())
            .and_modify(|balance| *balance -= amount);
        self.get_mut().total_supply -= amount;
        msg::reply(
            FTTransfer {
                from: msg::source(),
                to: ZERO_ID,
                amount,
            },
            0,
        )
        .expect("Error during a reply with FTEvent::FTTransfer");
    }

    /// Transfer `amount` of token
    ///
    /// Requirements:
    /// * Only the token owner or approved account can call that action
    /// * `from` MUST have enough tokens
    /// * `from` and `to` MUST be non-zero addresses
    ///
    /// Arguments:
    /// * `from`: An account from which token will be transerred
    /// * `to`: An account to which token will be transferred
    /// * `amount`: The amount of token of be transferred
    fn transfer(&mut self, from: &ActorId, to: &ActorId, amount: u128) {
        if from == &ZERO_ID || to == &ZERO_ID {
            panic!("Zero addresses");
        };
        if !self.can_transfer(from, amount) {
            panic!("Not allowed to transfer")
        }
        if self
            .get()
            .balances
            .get(from)
            .expect("The account has no balance at all")
            < &amount
        {
            panic!("Amount exceeds account's balance");
        }
        self.get_mut()
            .balances
            .entry(*from)
            .and_modify(|balance| *balance -= amount);
        self.get_mut()
            .balances
            .entry(*to)
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);
        msg::reply(
            FTTransfer {
                from: *from,
                to: *to,
                amount,
            },
            0,
        )
        .expect("Error during a reply with FTEvent::FTTransfer");
    }

    /// Gives a right to another account to manage the `amount` of token
    ///
    /// Requirements:
    /// * Only the token owner can call that action
    /// * `to` MUST be a non-zero account
    ///
    /// Arguments:
    /// * `to`: An account that will be approved to manage the indicated amount of token
    /// * `amount`: The amount of tokens to be approved
    fn approve(&mut self, to: &ActorId, amount: u128) {
        if to == &ZERO_ID {
            panic!("Approve to zero address");
        }
        self.get_mut()
            .allowances
            .entry(msg::source())
            .or_default()
            .insert(*to, amount);
        msg::reply(
            FTApproval {
                from: msg::source(),
                to: *to,
                amount,
            },
            0,
        )
        .expect("Error during a reply with FTEvent::FTApproval");
    }

    /// Checks whether it is possible to perform a transfer
    fn can_transfer(&mut self, from: &ActorId, amount: u128) -> bool {
        if let Some(allowed_amount) = self
            .get()
            .allowances
            .get(from)
            .and_then(|m| m.get(&msg::source()))
        {
            if allowed_amount >= &amount {
                self.get_mut().allowances.entry(*from).and_modify(|m| {
                    m.entry(msg::source()).and_modify(|a| *a -= amount);
                });
                return true;
            }
        }
        false
    }
}
