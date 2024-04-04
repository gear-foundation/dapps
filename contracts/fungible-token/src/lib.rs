#![no_std]

use fungible_token_io::*;
use gstd::{
    collections::{hash_map::Entry, HashMap, HashSet},
    exec, msg,
    prelude::*,
    ActorId,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Default)]
struct FungibleToken {
    /// Name of the token.
    name: String,
    /// Symbol of the token.
    symbol: String,
    /// Token's decimals.
    decimals: u8,
    /// Map to hold balances of token holders.
    balances: HashMap<ActorId, u128>,
    /// Map to hold allowance information of token holders.
    allowances: HashMap<ActorId, HashMap<ActorId, u128>>,
    /// Mapping of executed transactions to the time they are valid.
    tx_ids: HashMap<(ActorId, TxId), ValidUntil>,
    /// Mapping of accounts to their transaction IDs.
    account_to_tx_ids: HashMap<ActorId, HashSet<TxId>>,
    /// Configuration parameters for the fungible token contract.
    config: Config,
    admins: Vec<ActorId>,
}

static mut FUNGIBLE_TOKEN: Option<FungibleToken> = None;

impl FungibleToken {
    fn add_admin(&mut self, admin_id: &ActorId) -> Result<FTReply, FTError> {
        let source = msg::source();
        if !self.admins.contains(&source) {
            return Err(FTError::NotAdmin);
        }
        if self.admins.contains(admin_id) {
            return Err(FTError::AdminAlreadyExists);
        }
        self.admins.push(*admin_id);
        Ok(FTReply::AdminAdded {
            admin_id: *admin_id,
        })
    }

    fn delete_admin(&mut self, admin_id: &ActorId) -> Result<FTReply, FTError> {
        let source = msg::source();
        if !self.admins.contains(&source) {
            return Err(FTError::NotAdmin);
        }

        if admin_id == &source {
            return Err(FTError::CantDeleteYourself);
        }

        self.admins.retain(|acc| acc != admin_id);
        Ok(FTReply::AdminRemoved {
            admin_id: *admin_id,
        })
    }
    fn transfer(
        &mut self,
        tx_id: Option<TxId>,
        from: &ActorId,
        to: &ActorId,
        amount: u128,
    ) -> Result<FTReply, FTError> {
        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        if let Some(tx_id) = tx_id {
            self.clear_outdated_tx_ids(&msg_source, block_timestamp);
            self.check_tx_id(tx_id, &msg_source)?;
        }

        if *from == ActorId::zero() || *to == ActorId::zero() {
            return Err(FTError::ZeroAddress);
        };

        self.check_balance(&from, amount)?;

        self.can_transfer(&msg_source, &from, amount)?;

        self.balances
            .entry(*from)
            .and_modify(|balance| *balance -= amount);
        self.balances
            .entry(*to)
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);

        self.set_tx_id_status(
            tx_id,
            &msg_source,
            block_timestamp + self.config.tx_storage_period,
        );

        Ok(FTReply::Transferred {
            from: *from,
            to: *to,
            amount,
        })
    }

    /// Executed on receiving `fungible-token-messages::ApproveInput`.
    fn approve(
        &mut self,
        tx_id: Option<TxId>,
        to: &ActorId,
        amount: u128,
    ) -> Result<FTReply, FTError> {
        if *to == ActorId::zero() {
            return Err(FTError::ZeroAddress);
        }
        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        if let Some(tx_id) = tx_id {
            self.clear_outdated_tx_ids(&msg_source, block_timestamp);
            self.check_tx_id(tx_id, &msg_source)?;
        }
        self.allowances
            .entry(msg_source)
            .or_default()
            .insert(*to, amount);
        self.set_tx_id_status(
            tx_id,
            &msg_source,
            block_timestamp + self.config.tx_storage_period,
        );
        Ok(FTReply::Approved {
            from: msg_source,
            to: *to,
            amount,
        })
    }

    fn check_balance(&self, account: &ActorId, amount: u128) -> Result<(), FTError> {
        if *self.balances.get(account).unwrap_or(&0) < amount {
            return Err(FTError::NotEnoughBalance);
        }
        Ok(())
    }

    fn can_transfer(
        &mut self,
        source: &ActorId,
        from: &ActorId,
        amount: u128,
    ) -> Result<(), FTError> {
        if from != source {
            if let Some(allowed_amount) = self.allowances.get(from).and_then(|m| m.get(source)) {
                if allowed_amount >= &amount {
                    self.allowances.entry(*from).and_modify(|m| {
                        m.entry(*source).and_modify(|a| *a -= amount);
                    });
                } else {
                    return Err(FTError::NotAllowedToTransfer);
                }
            } else {
                return Err(FTError::NotAllowedToTransfer);
            }
        }
        Ok(())
    }

    fn set_tx_id_status(
        &mut self,
        tx_id: Option<TxId>,
        account: &ActorId,
        valid_until: ValidUntil,
    ) {
        if let Some(tx_id) = tx_id {
            self.tx_ids.insert((*account, tx_id), valid_until);
        }
    }

    fn check_tx_id(&self, tx_id: TxId, account: &ActorId) -> Result<(), FTError> {
        if self.tx_ids.get(&(*account, tx_id)).is_some() {
            return Err(FTError::TxAlreadyExists);
        }

        Ok(())
    }

    fn clear_outdated_tx_ids(&mut self, account: &ActorId, block_timestamp: u64) {
        if let Entry::Occupied(mut tx_ids) = self.account_to_tx_ids.entry(*account) {
            let tx_ids_cloned = tx_ids.get().clone();
            for tx_id in tx_ids_cloned {
                let valid_until = self.tx_ids.get(&(*account, tx_id)).expect("Cant be None");
                if block_timestamp > *valid_until {
                    self.tx_ids.remove(&(*account, tx_id));
                    tx_ids.get_mut().remove(&tx_id);
                }
            }
            if tx_ids.get().is_empty() {
                tx_ids.remove_entry();
            }
        }
    }
}

#[no_mangle]
extern fn handle() {
    let action: FTAction = msg::load().expect("Could not load Action");
    let ft: &mut FungibleToken = unsafe {
        FUNGIBLE_TOKEN
            .as_mut()
            .expect("The contract is not initialized")
    };
    let reply = match action {
        FTAction::AddAdmin { admin_id } => ft.add_admin(&admin_id),
        FTAction::DeleteAdmin { admin_id } => ft.delete_admin(&admin_id),
        FTAction::Transfer {
            tx_id,
            from,
            to,
            amount,
        } => ft.transfer(tx_id, &from, &to, amount),
        FTAction::Approve { tx_id, to, amount } => ft.approve(tx_id, &to, amount),
        FTAction::BalanceOf(account) => {
            let balance = ft.balances.get(&account).unwrap_or(&0);
            Ok(FTReply::Balance(*balance))
        }
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}

#[no_mangle]
extern fn init() {
    let init_config: InitConfig = msg::load().expect("Unable to decode InitConfig");
    let ft = FungibleToken {
        name: init_config.name,
        symbol: init_config.symbol,
        decimals: init_config.decimals,
        config: init_config.config,
        ..Default::default()
    };
    unsafe { FUNGIBLE_TOKEN = Some(ft) };
    msg::reply(FTReply::Initialized, 0).expect("Error in sending a reply");
}

#[no_mangle]
extern fn state() {
    let token = unsafe {
        FUNGIBLE_TOKEN
            .take()
            .expect("Unexpected: Error in getting contract state")
    };
    let query: Query = msg::load().expect("Unable to decode the query");
    let reply = match query {
        Query::Name => QueryReply::Name(token.name),
        Query::Symbol => QueryReply::Symbol(token.symbol),
        Query::Decimals => QueryReply::Decimals(token.decimals),
        Query::BalanceOf(account) => {
            let balance = if let Some(balance) = token.balances.get(&account) {
                *balance
            } else {
                0
            };
            QueryReply::Balance(balance)
        }
        Query::AllowanceOfAccount {
            account,
            approved_account,
        } => {
            let allowance = if let Some(allowance) = token
                .allowances
                .get(&account)
                .and_then(|m| m.get(&approved_account))
            {
                *allowance
            } else {
                0
            };
            QueryReply::AllowanceOfAccount(allowance)
        }
        Query::Admins => QueryReply::Admins(token.admins),
        Query::GetTxValidityTime { account, tx_id } => {
            let valid_until = token.tx_ids.get(&(account, tx_id)).unwrap_or(&0);
            QueryReply::TxValidityTime(*valid_until)
        }
        Query::GetTxIdsForAccount { account } => {
            let tx_ids: Vec<TxId> =
                if let Some(tx_ids) = token.account_to_tx_ids.get(&account).cloned() {
                    tx_ids.into_iter().collect()
                } else {
                    Vec::new()
                };
            QueryReply::TxIdsForAccount { tx_ids }
        }
    };
    msg::reply(reply, 0).expect("Error on sharinf state");
}
