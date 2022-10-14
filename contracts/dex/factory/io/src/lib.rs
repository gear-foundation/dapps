#![no_std]
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

pub type FungibleId = ActorId;

/// Initializes a factory.
///
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFactory {
    /// The address that can actually set the fee.
    pub fee_to_setter: ActorId,
    /// Code hash to successfully deploy a pair with this contract.
    pub pair_code_hash: [u8; 32],
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryAction {
    /// Creates an exchange pair
    ///
    /// Deploys a pair exchange contract and saves the info about it.
    /// # Requirements:
    /// * both `FungibleId` MUST be non-zero addresss and represent the actual fungible-token contracts
    ///
    /// On success returns `FactoryEvery::PairCreated`
    CreatePair(FungibleId, FungibleId),

    /// Sets fee_to variable
    ///
    /// Sets an address where the fees will be sent.
    /// # Requirements:
    /// * `fee_to` MUST be non-zero address
    /// * action sender MUST be the same as `fee_to_setter` in this contract
    ///
    /// On success returns `FactoryEvery::FeeToSet`
    SetFeeTo(ActorId),

    /// Sets fee_to_setter variable
    ///
    /// Sets an address that will be able to change fee_to
    /// # Requirements:
    /// * `fee_to_setter` MUST be non-zero address
    /// * action sender MUST be the same as `fee_to_setter` in this contract
    ///
    /// On success returns `FactoryEvery::FeeToSetterSet`
    SetFeeToSetter(ActorId),

    /// Returns a `fee_to` variables.
    ///
    /// Just returns a variable `fee_to` from the state.
    ///
    /// On success returns `FactoryEvery::FeeTo`
    FeeTo,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    PairCreated {
        /// The first token address
        token_a: FungibleId,
        /// The second token address
        token_b: FungibleId,
        /// Pair address (the pair exchange contract).
        pair_address: ActorId,
        /// The amount of pairs that already were deployed though this factory.
        pairs_length: u32,
    },
    FeeToSet(ActorId),
    FeeToSetterSet(ActorId),
    FeeTo(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryStateQuery {
    FeeTo,
    FeeToSetter,
    PairAddress {
        token_a: FungibleId,
        token_b: FungibleId,
    },
    AllPairsLength,
    Owner,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryStateReply {
    FeeTo { address: ActorId },
    FeeToSetter { address: ActorId },
    PairAddress { address: ActorId },
    AllPairsLength { length: u32 },
    Owner { address: ActorId },
}
