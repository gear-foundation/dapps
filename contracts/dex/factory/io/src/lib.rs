#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub use dex_pair_io::FungibleId;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitFactory>;
    type Handle = InOut<FactoryAction, FactoryEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = State;
}

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Encode, Decode)]
pub struct State {
    /// CodeHash to deploy a pair contract from factory.
    pub pair_code_hash: [u8; 32],
    pub owner_id: ActorId,
    /// Who gets the fee
    pub fee_to: ActorId,
    pub fee_to_setter: ActorId,
    /// (tokenA, tokenB) -> pair_address mapping
    pub pairs: Vec<((ActorId, ActorId), ActorId)>,
}

#[doc(hidden)]
impl State {
    pub fn pair_address(self, token_a: FungibleId, token_b: FungibleId) -> ActorId {
        let (t1, t2) = if token_a > token_b {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };

        self.pairs
            .into_iter()
            .find_map(|(pair, address)| (pair == (t1, t2)).then_some(address))
            .unwrap_or_default()
    }

    pub fn all_pairs_length(self) -> u32 {
        self.pairs.len() as u32
    }
}

/// Initializes a factory.
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitFactory {
    /// The address that can actually set the fee.
    pub fee_to_setter: ActorId,
    /// Code hash to successfully deploy a pair with this contract.
    pub pair_code_hash: [u8; 32],
}

#[derive(Debug, Encode, Decode, TypeInfo)]
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
pub enum FactoryStateReply {
    FeeTo(ActorId),
    FeeToSetter(ActorId),
    PairAddress(ActorId),
    AllPairsLength(u32),
    Owner(ActorId),
}
