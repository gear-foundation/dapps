use gstd::{exec, prelude::*, ActorId};
use hashbrown::HashMap;
use supply_chain_io::*;

const MAX_NUMBER_OF_TXS: usize = 2usize.pow(16);

pub struct TransactionManager<T> {
    txs_for_actor: BTreeMap<u64, ActorId>,
    actors_for_tx: HashMap<ActorId, (u64, T, u64)>,

    tx_id_nonce: u64,
}

impl<T> Default for TransactionManager<T> {
    fn default() -> Self {
        Self {
            txs_for_actor: Default::default(),
            actors_for_tx: Default::default(),

            tx_id_nonce: Default::default(),
        }
    }
}

impl<T: PartialEq + Clone> TransactionManager<T> {
    fn inner_asquire_transaction(
        &mut self,
        kind: TransactionKind,
        msg_source: ActorId,
        check_action: T,
        timestamp: u64,
    ) -> Result<TransactionGuard<T>, TransactionCacheError> {
        let (tx_id, timestamp) = match kind {
            TransactionKind::New => {
                let id = self.tx_id_nonce;

                self.tx_id_nonce = id.wrapping_add(u8::MAX as _);

                if self.txs_for_actor.len() == MAX_NUMBER_OF_TXS {
                    let (tx, actor) = self
                        .txs_for_actor
                        .range(self.tx_id_nonce..)
                        .next()
                        .unwrap_or_else(|| self.txs_for_actor.first_key_value().unwrap());
                    let (tx, actor) = (*tx, *actor);

                    self.txs_for_actor.remove(&tx);
                    self.actors_for_tx.remove(&actor);
                }

                self.txs_for_actor.insert(id, msg_source);
                self.actors_for_tx
                    .insert(msg_source, (id, check_action, timestamp));

                (id, timestamp)
            }
            TransactionKind::Retry => {
                let (id, true_checked_action, timestamp) = self
                    .actors_for_tx
                    .get(&msg_source)
                    .ok_or(TransactionCacheError::TransactionNotFound)?;

                if check_action.ne(true_checked_action) {
                    return Err(TransactionCacheError::MismatchedAction);
                }

                (*id, *timestamp)
            }
        };

        Ok(TransactionGuard {
            manager: self,
            msg_source,
            tx_id,

            step: 0,

            timestamp,
        })
    }

    pub fn asquire_transaction(
        &mut self,
        kind: TransactionKind,
        msg_source: ActorId,
        check_action: T,
    ) -> Result<TransactionGuard<T>, TransactionCacheError> {
        Self::inner_asquire_transaction(self, kind, msg_source, check_action, 0)
    }

    pub fn asquire_transaction_with_timestamp(
        &mut self,
        kind: TransactionKind,
        msg_source: ActorId,
        check_action: T,
    ) -> Result<TransactionGuard<T>, TransactionCacheError> {
        Self::inner_asquire_transaction(
            self,
            kind,
            msg_source,
            check_action,
            exec::block_timestamp(),
        )
    }

    pub fn cached_actions(&self) -> Vec<(ActorId, T)> {
        self.actors_for_tx
            .iter()
            .map(|(actor, (_, action, _))| (*actor, action.clone()))
            .collect()
    }
}

pub struct TransactionGuard<'a, T> {
    manager: &'a mut TransactionManager<T>,
    msg_source: ActorId,
    tx_id: u64,

    step: u8,

    pub timestamp: u64,
}

impl<T> TransactionGuard<'_, T> {
    pub fn step(&mut self) -> Result<u64, TransactionCacheError> {
        let step = self.tx_id + self.step as u64;

        if let Some(next_step) = self.step.checked_add(1) {
            self.step = next_step;

            Ok(step)
        } else {
            Err(TransactionCacheError::StepOverflow)
        }
    }
}

impl<T> Drop for TransactionGuard<'_, T> {
    fn drop(&mut self) {
        self.manager.txs_for_actor.remove(&self.tx_id);
        self.manager.actors_for_tx.remove(&self.msg_source);
    }
}
