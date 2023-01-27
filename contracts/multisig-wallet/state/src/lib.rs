#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use multisig_wallet_io::*;

fn common_confirmations_count(
    state: &<ContractMetadata as Metadata>::State,
    transaction_id: TransactionId,
) -> Option<u32> {
    state
        .confirmations
        .iter()
        .find_map(|(tx_id, confirmations)| {
            (tx_id == &transaction_id).then_some(
                confirmations
                    .iter()
                    .filter(|confirmation| state.owners.contains(confirmation))
                    .count() as _,
            )
        })
}

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    // Returns number of confirmations of a transaction.
    // `transaction_id` Transaction ID.
    // Number of confirmations.
    fn confirmations_count(transaction_id: TransactionId, state: Self::State) -> Option<u32> {
        common_confirmations_count(&state, transaction_id)
    }

    // Returns total number of transactions after filers are applied.
    // `pending` Include pending transactions.
    // `executed` Include executed transactions.
    // Total number of transactions after filters are applied.
    fn transactions_count(pending_executed: PendingExecuted, state: Self::State) -> u32 {
        state
            .transactions
            .into_iter()
            .filter(|(_, tx)| {
                (pending_executed.0 && !tx.executed) || (pending_executed.1 && tx.executed)
            })
            .count() as _
    }

    // Returns list of owners.
    // List of owner addresses.
    fn owners(state: Self::State) -> Vec<ActorId> {
        state.owners
    }

    // Returns array with owner addresses, which confirmed transaction.
    // `transaction_id` Transaction ID.
    // Returns array of owner addresses.
    fn confirmations(transaction_id: TransactionId, state: Self::State) -> Option<Vec<ActorId>> {
        state
            .confirmations
            .into_iter()
            .find_map(|(tx_id, confirmations)| (tx_id == transaction_id).then_some(confirmations))
    }

    // Returns list of transaction IDs in defined range.
    // `from` Index start position of transaction array.
    // `to` Index end position of transaction array(not included).
    // `pending` Include pending transactions.
    // `executed` Include executed transactions.
    // `Returns` array of transaction IDs.
    fn transaction_ids(
        from_to_pending_executed: FromToPendingExecuted,
        state: Self::State,
    ) -> Vec<TransactionId> {
        let (from, to, pending, executed) = from_to_pending_executed;

        state
            .transactions
            .into_iter()
            .filter(|(_, tx)| (pending && !tx.executed) || (executed && tx.executed))
            .map(|(id, _)| id)
            .take(to as _)
            .skip(from as _)
            .collect()
    }

    // Returns the confirmation status of a transaction.
    // `transaction_id` Transaction ID.
    fn is_confirmed(transaction_id: TransactionId, state: Self::State) -> bool {
        let required = state.required;

        if let Some(count) = common_confirmations_count(&state, transaction_id) {
            count >= required
        } else {
            false
        }
    }

    // Returns the description of a transaction.
    // `transaction_id` Transaction ID.
    fn transaction_description(
        transaction_id: TransactionId,
        state: Self::State,
    ) -> Option<Option<String>> {
        state
            .transactions
            .into_iter()
            .find_map(|(tx_id, tx)| (tx_id == transaction_id).then_some(tx.description))
    }
}

pub type PendingExecuted = (bool, bool);
pub type FromToPendingExecuted = (u32, u32, bool, bool);
