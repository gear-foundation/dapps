#![no_std]

use gstd::{msg, prelude::*, ActorId};
use hashbrown::{HashMap, HashSet};
use nft_master_io::{NFTMasterAction, NFTMasterEvent, NFTMasterInit, NFTMasterState};

#[derive(Debug)]
struct NFTMaster {
    pub nfts: HashMap<ActorId, String>,
    pub operators: HashSet<ActorId>,
}

impl NFTMaster {
    pub fn new_with_operator(operator: &ActorId) -> Self {
        let mut operators = HashSet::new();
        operators.insert(*operator);

        NFTMaster {
            nfts: HashMap::new(),
            operators,
        }
    }

    pub fn add_nft_contract(
        &mut self,
        caller: &ActorId,
        nft_contract: &ActorId,
        meta: String,
    ) -> NFTMasterEvent {
        if !self.operators.contains(caller) {
            NFTMasterEvent::Error("Only operator can change nfts.".to_owned())
        } else if self.nfts.insert(*nft_contract, meta.clone()).is_some() {
            NFTMasterEvent::NFTContractUpdated {
                operator: *caller,
                nft_contract: *nft_contract,
                meta,
            }
        } else {
            NFTMasterEvent::NFTContractAdded {
                operator: *caller,
                nft_contract: *nft_contract,
                meta,
            }
        }
    }

    pub fn remove_nft_contract(
        &mut self,
        caller: &ActorId,
        nft_contract: &ActorId,
    ) -> NFTMasterEvent {
        if !self.operators.contains(caller) {
            NFTMasterEvent::Error("Only operator can remove nfts.".to_owned())
        } else if !self.nfts.contains_key(nft_contract) {
            NFTMasterEvent::Error("NFT does not exist.".to_owned())
        } else {
            self.nfts.remove(nft_contract);
            NFTMasterEvent::NFTContractDeleted {
                operator: *caller,
                nft_contract: *nft_contract,
            }
        }
    }

    pub fn add_operator(&mut self, caller: &ActorId, operator: &ActorId) -> NFTMasterEvent {
        if !self.operators.contains(caller) {
            NFTMasterEvent::Error("Only operator can add operators.".to_owned())
        } else if self.operators.contains(operator) {
            NFTMasterEvent::Error("Operator already exist.".to_owned())
        } else {
            self.operators.insert(*operator);
            NFTMasterEvent::OperatorAdded {
                operator: *caller,
                new_operator: *operator,
            }
        }
    }

    pub fn remove_operator(&mut self, caller: &ActorId, operator: &ActorId) -> NFTMasterEvent {
        if !self.operators.contains(caller) {
            NFTMasterEvent::Error("Only operator can remove operators.".to_owned())
        } else if !self.operators.contains(operator) {
            NFTMasterEvent::Error("Operator does not exist.".to_owned())
        } else {
            self.operators.remove(operator);
            NFTMasterEvent::OperatorRemoved {
                operator: *caller,
                removed_operator: *operator,
            }
        }
    }
}

impl From<&NFTMaster> for NFTMasterState {
    fn from(value: &NFTMaster) -> Self {
        NFTMasterState {
            nfts: value.nfts.iter().map(|(k, v)| (*k, v.clone())).collect(),
            operators: value.operators.iter().copied().collect(),
        }
    }
}

static mut NFT_MASTER: Option<NFTMaster> = None;

#[no_mangle]
extern fn init() {
    let _init: NFTMasterInit = msg::load().expect("Unable to decode `NFTMasterInit`.");
    let caller = msg::source();

    unsafe { NFT_MASTER = Some(NFTMaster::new_with_operator(&caller)) }
}

#[no_mangle]
extern fn handle() {
    let action: NFTMasterAction = msg::load().expect("Couldn't load `NFTMasterAction`.");
    let nft_master: &mut NFTMaster =
        unsafe { NFT_MASTER.as_mut().expect("Couldn't load `NFTMaster`.") };

    let caller = msg::source();

    let event = match action {
        NFTMasterAction::AddNFTContract { nft_contract, meta } => {
            nft_master.add_nft_contract(&caller, &nft_contract, meta)
        }
        NFTMasterAction::RemoveNFTContract { nft_contract } => {
            nft_master.remove_nft_contract(&caller, &nft_contract)
        }
        NFTMasterAction::AddOperator { operator } => nft_master.add_operator(&caller, &operator),
        NFTMasterAction::RemoveOperator { operator } => {
            nft_master.remove_operator(&caller, &operator)
        }
    };

    msg::reply(event, 0).expect("Failed to encode or reply with `NFTMasterEvent`.");
}

#[no_mangle]
extern fn state() {
    msg::reply(
        unsafe {
            let nft_master = NFT_MASTER
                .as_ref()
                .expect("Uninitialized `NFTMaster` state.");
            let nft_master_state: NFTMasterState = nft_master.into();
            nft_master_state
        },
        0,
    )
    .expect("Failed to share `NFTMarketState`.");
}
