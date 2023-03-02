#![no_std]

use escrow_io::*;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use primitive_types::U256;

#[metawasm]
pub mod metafns {
    pub type State = <EscrowMetadata as Metadata>::State;

    pub fn info(state: State, wallet_id: U256) -> Wallet {
        let (_, wallet) = *state
            .wallets
            .iter()
            .find(|(id, _)| id == &wallet_id)
            .unwrap_or_else(|| panic!("Wallet with the {wallet_id} ID doesn't exist"));

        wallet
    }

    pub fn created_wallets(state: State) -> Vec<(WalletId, Wallet)> {
        state
            .wallets
            .iter()
            .map(|(wallet_id, wallet)| (*wallet_id, *wallet))
            .collect()
    }
}
