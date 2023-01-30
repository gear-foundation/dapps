#![no_std]

use escrow_io::*;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use primitive_types::U256;

#[metawasm]
pub trait Metawasm {
    type State = <EscrowMetadata as Metadata>::State;

    fn info(wallet_id: U256, state: Self::State) -> Wallet {
        let (_, wallet) = *state
            .wallets
            .iter()
            .find(|(id, _)| id == &wallet_id)
            .unwrap_or_else(|| panic!("Wallet with the {wallet_id} ID doesn't exist"));

        wallet
    }

    fn created_wallets(state: Self::State) -> Vec<(WalletId, Wallet)> {
        state
            .wallets
            .iter()
            .map(|(wallet_id, wallet)| (*wallet_id, *wallet))
            .collect()
    }
}
