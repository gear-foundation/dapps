#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct NFTMasterMetadata;

impl Metadata for NFTMasterMetadata {
    type Init = In<NFTMasterInit>;
    type Handle = InOut<NFTMasterAction, NFTMasterEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = NFTMasterState;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTMasterInit {}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTMasterAction {
    AddNFTContract { nft_contract: ActorId, meta: String },
    RemoveNFTContract { nft_contract: ActorId },
    AddOperator { operator: ActorId },
    RemoveOperator { operator: ActorId },
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTMasterEvent {
    NFTContractAdded {
        operator: ActorId,
        nft_contract: ActorId,
        meta: String,
    },
    NFTContractUpdated {
        operator: ActorId,
        nft_contract: ActorId,
        meta: String,
    },
    NFTContractDeleted {
        operator: ActorId,
        nft_contract: ActorId,
    },
    OperatorAdded {
        operator: ActorId,
        new_operator: ActorId,
    },
    OperatorRemoved {
        operator: ActorId,
        removed_operator: ActorId,
    },
    Error(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTMasterState {
    pub nfts: Vec<(ActorId, String)>,
    pub operators: Vec<ActorId>,
}
