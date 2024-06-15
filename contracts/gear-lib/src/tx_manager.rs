//! The transaction manager.
//!
//! With the advent of complex asynchronous smart contracts, like
//! [SFT](https://github.com/gear-dapps/sharded-fungible-token), the transaction
//! caching was introduced. It allows transaction data to be saved on different
//! stages, and a failed transaction to be reexecuted whilst skipping completed
//! stages, thereby saving gas. Most often the reason for the failed transaction
//! is the lack of gas. Unfortunately, this algorithm isn't too user-friendly
//! because users of contracts with caching must track cached
//! [`TransactionId`]s, and increment their number on each successful execution.
//! [`TransactionManager`] is intended to help solve this problem.
//!
//! In simple words, [`TransactionManager`] stores pairs of [`msg::source()`] &
//! some data that a contract developer wants to save for this
//! [`msg::source()`]. The manager has a limit at which it starts to replace old
//! pairs with new ones. While doing all this, it tracks transaction identifiers
//! (or more specifically slices of identifiers) for all pairs. Thus, having
//! only [`msg::source()`], it's possible to acquire [`TransactionGuard`] with
//! [`Stepper`] inside, and by calling [`Stepper::step()`] always get the
//! correct [`TransactionId`] regardless of whether the transaction is new or
//! cached.
//!
//! [`msg::source()`]: gstd::msg::source

use ahash::AHasher;
use core::num::{NonZeroU32, NonZeroUsize};
use gstd::{hash::BuildHasherDefault, prelude::*, ActorId};
use indexmap::{map::MutableKeys, IndexMap};
use offset::Offset;

mod offset;

/// The default transaction limit.
///
/// The contract storage is limited, so cached transactions can't take up all
/// the space. Note that although this limit may be enough for most example
/// cases, the real contract structure may still be very different, so it's
/// recommended to calculate an individual limit for each contract, and set it
/// with [`TransactionManager::new_with_custom_limit()`].
pub const DEFAULT_TX_LIMIT: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(2u32.pow(16)) };
/// The maximum transaction limit.
///
/// A transaction limit mustn't be more than [`u32::MAX`]` / `[`u8::MAX`]` - 1`.
/// With the current contract memory limit (32 MB), it's impossible to store
/// even the half of this amount, so consider this just as an additional
/// restriction.
///
/// The reason for it is the [`TransactionManager`]'s logic for
/// [`TransactionId`] traversing. The manager divides available
/// [`TransactionId`]s by [`u8::MAX`], saves only division indexes, and
/// multiplies them by [`u8::MAX`] to get actual [`TransactionId`]s. Hence to
/// avoid the [`u32`] overflow, the maximum amount of cached transactions
/// multiplied by [`u8::MAX`] mustn't be more than
/// [`u32::MAX`]` / `[`u8::MAX`]` - 1`.
pub const MAX_TX_LIMIT: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(Offset::MAX) };

/// A transaction identifier.
// TODO: change to `u32` after the SFT refactor.
pub type TransactionId = u64;

impl<T> Default for TransactionManager<T> {
    fn default() -> Self {
        Self {
            actors_for_tx: IndexMap::default(),
            tx_limit: DEFAULT_TX_LIMIT.try_into().unwrap(),
            cursor: 0,
            offset: 0.try_into().unwrap(),
        }
    }
}

/// The transaction manager.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransactionManager<T> {
    actors_for_tx: IndexMap<ActorId, T, BuildHasherDefault<AHasher>>,
    tx_limit: NonZeroUsize,
    cursor: usize,
    offset: Offset,
}

impl<T> TransactionManager<T> {
    /// Creates the manager with [`DEFAULT_TX_LIMIT`].
    ///
    /// To create it with a custom limit, use
    /// [`TransactionManager::new_with_custom_limit()`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates the manager with custom `tx_limit`.
    ///
    /// # Errors
    /// [`TransactionManagerError::Overflow`] if `tx_limit` > [`MAX_TX_LIMIT`].
    ///
    /// To create the manager with the default limit, use
    /// [`TransactionManager::new()`].
    pub fn new_with_custom_limit(tx_limit: NonZeroU32) -> Result<Self, TransactionManagerError> {
        if tx_limit > MAX_TX_LIMIT {
            Err(TransactionManagerError::Overflow)
        } else {
            Ok(Self {
                tx_limit: tx_limit.try_into().unwrap(),
                ..Self::new()
            })
        }
    }

    /// Acquires the transaction for a given [`msg::source()`].
    ///
    /// Important notes:
    /// - Only one transaction for each `msg_source` is cached. Hence an attempt
    ///   to save a [new](TransactionKind::New) transaction over a failed one will
    ///   delete the failed one, so it'll be **impossible** to
    ///   [retry](TransactionKind::Retry) the latter.
    /// - There's no guarantee every underprocessed asynchronous action will
    ///   result in a cached transaction. Usually caching occurs after the first
    ///   blocking `.await` during action processing.
    /// - The cache memory has a limit, so when it's reached every oldest cached
    ///   transaction is replaced with a new one. See also [`DEFAULT_TX_LIMIT`].
    ///
    /// # Errors
    /// [`TransactionManagerError::TransactionNotFound`] if `kind` is
    /// [`TransactionKind::Retry`], and transaction for given `msg_source`
    /// wasn't found.
    ///
    /// # Panics
    /// If `msg_source` is [`ActorId::zero()`]. [`msg::source()`] can't be
    /// [`ActorId::zero()`] because the manager use it for shadowing old
    /// transaction data.
    ///
    /// [`msg::source()`]: gstd::msg::source
    pub fn acquire_transaction(
        &mut self,
        msg_source: ActorId,
        kind: TransactionKind<T>,
    ) -> Result<TransactionGuard<'_, T>, TransactionManagerError> {
        assert!(
            !msg_source.is_zero(),
            "`ActorId::zero()` in `msg_source` is forbidden in the transaction manager"
        );

        let (tx_id, tx_data) = match kind {
            TransactionKind::New(data_to_cache) => {
                if self.actors_for_tx.len() < self.tx_limit.get() {
                    self.hoard(msg_source, data_to_cache)
                } else {
                    self.cycle(msg_source, data_to_cache)
                }
            }
            TransactionKind::Retry => {
                let (tx_index, _, tx_data) = self
                    .actors_for_tx
                    .get_full_mut(&msg_source)
                    .ok_or(TransactionManagerError::TransactionNotFound)?;
                let tx_id = if tx_index < self.cursor {
                    self.offset
                        .wrapping_add((tx_index as u32).try_into().unwrap())
                } else {
                    self.offset.wrapping_sub(
                        (self.tx_limit.get() as u32 - tx_index as u32)
                            .try_into()
                            .unwrap(),
                    )
                };

                (tx_id.get(), tx_data)
            }
        };

        Ok(TransactionGuard {
            tx_data: TransactionData(tx_data),
            stepper: Stepper {
                tx_id: tx_id * 255,
                step: 0,
            },
        })
    }

    /// Returns pairs of [`msg::source()`](gstd::msg::source) & cached
    /// transaction data in order from oldest to newest.
    ///
    /// Can be used to generate a list of cached transaction for the `state()`
    /// entry point.
    pub fn cached_transactions(&self) -> impl Iterator<Item = (&ActorId, &T)> {
        (self.cursor..self.actors_for_tx.len())
            .chain(0..self.cursor)
            .filter_map(|index| {
                let (actor, tx_data) = self.actors_for_tx.get_index(index).unwrap();

                (self.actors_for_tx.get_index_of(actor).unwrap() == index)
                    .then_some((actor, tx_data))
            })
    }

    fn hoard(&mut self, msg_source: ActorId, data_to_cache: T) -> (u32, &mut T) {
        // Hoarding mode. The manager will cache transactions until it hits the
        // set limit.

        let (mut tx_index, old_tx_data_option) =
            self.actors_for_tx.insert_full(msg_source, data_to_cache);

        if let Some(old_tx_data) = old_tx_data_option {
            // Inserting the old data at the end of the map.
            let (last_tx_index, _) = self.actors_for_tx.insert_full(ActorId::zero(), old_tx_data);
            let (_, key, _) = self.actors_for_tx.get_full_mut2(&ActorId::zero()).unwrap();

            // Invalidating the old data key so that the data can't be looked
            // up.
            *key = msg_source;

            // Moving the new data to the end.
            self.actors_for_tx.swap_indices(tx_index, last_tx_index);

            tx_index = last_tx_index;
        }

        self.update_cursor_and_offset();

        (tx_index as u32, &mut self.actors_for_tx[tx_index])
    }

    fn cycle(&mut self, msg_source: ActorId, data_to_cache: T) -> (u32, &mut T) {
        // Cycling mode. The manager will cycle through saved transactions and
        // overwrite oldest ones with new ones.

        let (mut tx_index, old_tx_data_option) =
            self.actors_for_tx.insert_full(msg_source, data_to_cache);

        if old_tx_data_option.is_some() {
            let (key, _) = self.actors_for_tx.get_index_mut2(self.cursor).unwrap();

            // Invalidating the old data key so that the data can't be looked
            // up.
            *key = msg_source;

            // Moving the new data to the end (the current cursor position).
            self.actors_for_tx.swap_indices(tx_index, self.cursor);
        } else {
            // Swapping the old data with the new one, and popping the old data
            // off.
            self.actors_for_tx.swap_remove_index(self.cursor);
        }

        tx_index = self.cursor;

        let tx_id = self
            .offset
            .wrapping_add((tx_index as u32).try_into().unwrap());

        self.update_cursor_and_offset();

        (tx_id.get(), &mut self.actors_for_tx[tx_index])
    }

    fn update_cursor_and_offset(&mut self) {
        let next_cursor_index = self.cursor + 1;

        if next_cursor_index < self.tx_limit.get() {
            self.cursor = next_cursor_index;
        } else {
            self.cursor = 0;
            self.offset = self
                .offset
                .wrapping_add((self.tx_limit.get() as u32).try_into().unwrap());
        }
    }
}

/// The kind of a transaction to get from [`TransactionManager`].
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TransactionKind<T> {
    /// A new transaction with some data to be cached.
    ///
    /// Keep the data as compact as possible because it'll stay in the contract
    /// memory until the transaction limit for [`TransactionManager`] is
    /// reached and the data overwritten with new transaction data.
    New(T),

    /// A cached transaction.
    Retry,
}

impl<T: Default> Default for TransactionKind<T> {
    fn default() -> Self {
        Self::New(T::default())
    }
}

/// The transaction manager error variants.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TransactionManagerError {
    /// There's no cached transaction for given
    /// [`msg::source()`](gstd::msg::source()). The reason may be a
    /// transaction's action wasn't asynchronous or just wasn't cached, or a
    /// cached transaction was removed because it was too old.
    TransactionNotFound,
    /// [`TransactionData`] failed a check in one of its methods.
    MismatchedTxData,
    /// See [`TransactionManager::new_with_custom_limit()`] or
    /// [`Stepper::step()`].
    Overflow,
}

/// A transaction guard.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionGuard<'tx, T> {
    pub tx_data: TransactionData<'tx, T>,
    pub stepper: Stepper,
}

/// Transaction data.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TransactionData<'tx, T>(pub &'tx mut T);

impl<T> TransactionData<'_, T> {
    /// Checks transaction data with a given closure and returns a mutable
    /// reference to the data or a part of it.
    ///
    /// ```
    /// use gear_lib::tx_manager::TransactionData;
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum SomeData {
    ///     One(u8),
    ///     #[allow(dead_code)]
    ///     Two,
    /// }
    ///
    /// let mut some_data = SomeData::One(123);
    /// let mut transaction_data = TransactionData(&mut some_data);
    ///
    /// let number = transaction_data
    ///     .check_and_get_tx_data(|tx_data| {
    ///         if let SomeData::One(ref mut number) = *tx_data {
    ///             Some(number)
    ///         } else {
    ///             None
    ///         }
    ///     })
    ///     .unwrap();
    ///
    /// assert_eq!(*number, 123);
    ///
    /// *number = 23;
    ///
    /// assert_eq!(some_data, SomeData::One(23));
    /// ```
    ///
    /// # Errors
    /// [`TransactionManagerError::MismatchedTxData`] if a check resulted in
    /// [`None`].
    pub fn check_and_get_tx_data<D>(
        &mut self,
        mut check: impl FnMut(&mut T) -> Option<&mut D>,
    ) -> Result<&mut D, TransactionManagerError> {
        check(self.0).ok_or(TransactionManagerError::MismatchedTxData)
    }

    /// Checks transaction data with a given closure.
    ///
    /// ```
    /// use gear_lib::tx_manager::{TransactionData, TransactionManagerError};
    ///
    /// #[derive(PartialEq)]
    /// enum SomeData {
    ///     One(u8),
    ///     #[allow(dead_code)]
    ///     Two,
    /// }
    ///
    /// let mut some_data = SomeData::One(123);
    /// let transaction_data = TransactionData(&mut some_data);
    ///
    /// assert_eq!(
    ///     transaction_data.check_tx_data(|tx_data| SomeData::One(123) == *tx_data),
    ///     Ok(())
    /// );
    /// assert_eq!(
    ///     transaction_data.check_tx_data(|tx_data| SomeData::One(1) == *tx_data),
    ///     Err(TransactionManagerError::MismatchedTxData)
    /// );
    /// ```
    ///
    /// # Errors
    /// [`TransactionManagerError::MismatchedTxData`] if a check resulted in
    /// [`false`].
    pub fn check_tx_data(
        &self,
        mut check: impl FnMut(&T) -> bool,
    ) -> Result<(), TransactionManagerError> {
        if check(self.0) {
            Ok(())
        } else {
            Err(TransactionManagerError::MismatchedTxData)
        }
    }
}

/// A [`TransactionId`] tracker for the current transaction.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stepper {
    tx_id: u32,
    step: u8,
}

impl Stepper {
    /// Gets the next [`TransactionId`] for the current transaction.
    ///
    /// The current limit for steps is [`u8::MAX`]. Since there are usually far
    /// fewer than [`u8::MAX`] interactions between contracts per action, this
    /// should be sufficient.
    ///
    /// # Errors
    /// [`TransactionManagerError::Overflow`] if the limit for [`Stepper`] was
    /// exceeded.
    pub fn step(&mut self) -> Result<TransactionId, TransactionManagerError> {
        let step = self.tx_id + u32::from(self.step);

        if let Some(next_step) = self.step.checked_add(1) {
            self.step = next_step;

            Ok(TransactionId::from(step))
        } else {
            Err(TransactionManagerError::Overflow)
        }
    }
}

/// The generic action type for use with [`TransactionManager`].
#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Action<T> {
    pub action: T,
    pub kind: ActionKind,
}

impl<T> Action<T> {
    pub const fn new(action: T) -> Self {
        Self {
            action,
            kind: ActionKind::New,
        }
    }

    pub fn to_retry(self) -> Self {
        Self {
            action: self.action,
            kind: ActionKind::Retry,
        }
    }
}

/// A kind of [`Action`].
///
/// The same as [`TransactionKind`], but without the data field. Should be used
/// instead of [`TransactionKind`] in a public interface.
#[derive(
    Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash,
)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ActionKind {
    #[default]
    New,
    Retry,
}

impl ActionKind {
    /// Converts [`ActionKind`] to [`TransactionKind`].
    pub fn to_tx_kind<T>(self, tx_data: T) -> TransactionKind<T> {
        match self {
            Self::New => TransactionKind::New(tx_data),
            Self::Retry => TransactionKind::Retry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_custom() {
        TransactionManager::<()>::new_with_custom_limit(1.try_into().unwrap()).unwrap();
        TransactionManager::<()>::new_with_custom_limit(MAX_TX_LIMIT).unwrap();
        assert_eq!(
            TransactionManager::<()>::new_with_custom_limit(MAX_TX_LIMIT.saturating_add(1)),
            Err(TransactionManagerError::Overflow)
        );
    }

    #[test]
    #[should_panic = "`ActorId::zero()` in `msg_source` is forbidden in the transaction manager"]
    fn forbidden_zero() {
        TransactionManager::<()>::new()
            .acquire_transaction(ActorId::zero(), TransactionKind::default())
            .unwrap();
    }

    #[test]
    fn extra_small_limit() {
        let tx_limit: NonZeroUsize = 1.try_into().unwrap();
        let mut manager =
            TransactionManager::<u8>::new_with_custom_limit(tx_limit.try_into().unwrap()).unwrap();
        let cursor = 0;
        let mut guard;

        for offset in 1..101 {
            guard = manager
                .acquire_transaction(ActorId::from(1), TransactionKind::New(228))
                .unwrap();

            assert_eq!(
                (*guard.tx_data.0, guard.stepper.tx_id),
                (228, (offset - 1) * 255)
            );

            guard = manager
                .acquire_transaction(ActorId::from(1), TransactionKind::Retry)
                .unwrap();

            assert_eq!(
                (*guard.tx_data.0, guard.stepper.tx_id),
                (228, (offset - 1) * 255)
            );
            assert_eq!(
                manager,
                TransactionManager {
                    actors_for_tx: IndexMap::from_iter([(ActorId::from(1), 228)]),
                    tx_limit,
                    cursor,
                    offset: offset.try_into().unwrap(),
                }
            );
            assert!(manager
                .cached_transactions()
                .eq([(&ActorId::from(1), &228)]));
        }

        manager.offset = (MAX_TX_LIMIT.get() - 1).try_into().unwrap();

        guard = manager
            .acquire_transaction(ActorId::from(1), TransactionKind::New(123))
            .unwrap();

        assert_eq!(
            (*guard.tx_data.0, guard.stepper.tx_id),
            (123, u32::MAX - 255 * 2)
        );
        assert_eq!(
            manager,
            TransactionManager {
                actors_for_tx: IndexMap::from_iter([(ActorId::from(1), 123)]),
                tx_limit,
                cursor,
                offset: MAX_TX_LIMIT.get().try_into().unwrap(),
            }
        );

        guard = manager
            .acquire_transaction(ActorId::from(1), TransactionKind::New(123))
            .unwrap();

        assert_eq!(
            (*guard.tx_data.0, guard.stepper.tx_id),
            (123, u32::MAX - 255)
        );
        assert_eq!(
            manager,
            TransactionManager {
                actors_for_tx: IndexMap::from_iter([(ActorId::from(1), 123)]),
                tx_limit,
                cursor,
                offset: 0.try_into().unwrap(),
            }
        );
        assert!(manager
            .cached_transactions()
            .eq([(&ActorId::from(1), &123)]));
    }

    #[test]
    fn small_limit() {
        let tx_limit: NonZeroUsize = 5.try_into().unwrap();
        let mut manager =
            TransactionManager::<u8>::new_with_custom_limit(tx_limit.try_into().unwrap()).unwrap();
        let mut guard;
        let mut txs = vec![];

        for offset in 1..6 {
            guard = manager
                .acquire_transaction(ActorId::from(1), TransactionKind::New(228))
                .unwrap();

            txs.push((ActorId::from(1), 228));

            assert_eq!(
                (*guard.tx_data.0, guard.stepper.tx_id),
                (228, (offset - 1) * 255)
            );

            guard = manager
                .acquire_transaction(ActorId::from(1), TransactionKind::Retry)
                .unwrap();

            assert_eq!(
                (*guard.tx_data.0, guard.stepper.tx_id),
                (228, (offset - 1) * 255)
            );
            assert_eq!(
                (manager.tx_limit, manager.cursor, manager.offset),
                (
                    5.try_into().unwrap(),
                    offset as usize % tx_limit,
                    (offset - offset % tx_limit.get() as u32)
                        .try_into()
                        .unwrap()
                )
            );
            assert_eq!(
                manager
                    .actors_for_tx
                    .iter()
                    .map(|(&k, &v)| (k, v))
                    .collect::<Vec<_>>(),
                txs
            );
            assert!(manager
                .cached_transactions()
                .eq(iter::once((&ActorId::from(1), &228))));
        }

        let mut guard = manager
            .acquire_transaction(ActorId::from(2), TransactionKind::New(77))
            .unwrap();

        assert_eq!((*guard.tx_data.0, guard.stepper.tx_id), (77, 5 * 255));

        guard = manager
            .acquire_transaction(ActorId::from(2), TransactionKind::Retry)
            .unwrap();

        assert_eq!((*guard.tx_data.0, guard.stepper.tx_id), (77, 5 * 255));
        assert_eq!(
            (manager.tx_limit, manager.cursor, manager.offset),
            (tx_limit, 1, (tx_limit.get() as u32).try_into().unwrap())
        );
        assert!(manager
            .actors_for_tx
            .iter()
            .eq(iter::once((&ActorId::from(2), &77))
                .chain(iter::repeat((&ActorId::from(1), &228)).take(4))));
        assert!(manager
            .cached_transactions()
            .eq([(&ActorId::from(1), &228), (&ActorId::from(2), &77)]));
    }

    #[test]
    fn borderline() {
        let mut manager = TransactionManager {
            actors_for_tx: IndexMap::from_iter([
                (ActorId::from(2), 234),
                (ActorId::from(3), 345),
                (ActorId::from(1), 123u16),
            ]),
            tx_limit: 3.try_into().unwrap(),
            cursor: 2,
            offset: 0.try_into().unwrap(),
        };

        assert!(manager.cached_transactions().eq([
            (&ActorId::from(1), &123),
            (&ActorId::from(2), &234),
            (&ActorId::from(3), &345),
        ]));

        let mut guard = manager
            .acquire_transaction(ActorId::from(1), TransactionKind::Retry)
            .unwrap();

        assert_eq!(
            (*guard.tx_data.0, guard.stepper.tx_id),
            (123, u32::MAX - 255)
        );

        guard = manager
            .acquire_transaction(ActorId::from(2), TransactionKind::Retry)
            .unwrap();

        assert_eq!((*guard.tx_data.0, guard.stepper.tx_id), (234, 0));

        guard = manager
            .acquire_transaction(ActorId::from(3), TransactionKind::Retry)
            .unwrap();

        assert_eq!((*guard.tx_data.0, guard.stepper.tx_id), (345, 255));

        guard = manager
            .acquire_transaction(ActorId::from(1), TransactionKind::New(7654))
            .unwrap();

        assert_eq!((*guard.tx_data.0, guard.stepper.tx_id), (7654, 255 * 2));
        assert!(manager.cached_transactions().eq([
            (&ActorId::from(2), &234),
            (&ActorId::from(3), &345),
            (&ActorId::from(1), &7654),
        ]));
    }
}
