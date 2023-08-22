use super::{ADMIN, NFT_MASTER_ID};
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use nft_master_io::{NFTMasterAction, NFTMasterEvent, NFTMasterInit, NFTMasterState};

pub trait NFTMasterMock {
    fn nft_master_mock(system: &System) -> Program;
    fn add_nft_contract(&self, from: u64, nft_contract: &ActorId, meta: &str, error: bool);
    fn remove_nft_contract(&self, from: u64, nft_contract: &ActorId, error: bool);
    fn add_operator(&self, from: u64, operator: &ActorId, error: bool);
    fn remove_operator(&self, from: u64, operator: &ActorId, error: bool);
    fn send_nft_master_tx(&self, from: u64, action: NFTMasterAction, error: bool);
    fn get_state(&self) -> NFTMasterState;
}

impl NFTMasterMock for Program<'_> {
    fn nft_master_mock(system: &System) -> Program {
        let nft_master = Program::current_with_id(system, NFT_MASTER_ID);
        assert!(!nft_master.send(ADMIN, NFTMasterInit {}).main_failed());

        nft_master
    }

    fn add_nft_contract(&self, from: u64, nft_contract: &ActorId, meta: &str, error: bool) {
        self.send_nft_master_tx(
            from,
            NFTMasterAction::AddNFTContract {
                nft_contract: *nft_contract,
                meta: meta.to_owned(),
            },
            error,
        )
    }

    fn remove_nft_contract(&self, from: u64, nft_contract: &ActorId, error: bool) {
        self.send_nft_master_tx(
            from,
            NFTMasterAction::RemoveNFTContract {
                nft_contract: *nft_contract,
            },
            error,
        )
    }

    fn add_operator(&self, from: u64, operator: &ActorId, error: bool) {
        self.send_nft_master_tx(
            from,
            NFTMasterAction::AddOperator {
                operator: *operator,
            },
            error,
        )
    }

    fn remove_operator(&self, from: u64, operator: &ActorId, error: bool) {
        self.send_nft_master_tx(
            from,
            NFTMasterAction::RemoveOperator {
                operator: *operator,
            },
            error,
        )
    }

    fn send_nft_master_tx(&self, from: u64, action: NFTMasterAction, error: bool) {
        let result = self.send(from, action);
        assert!(!result.main_failed());

        let maybe_error = result.log().iter().find_map(|log| {
            let mut payload = log.payload();
            if let Ok(NFTMasterEvent::Error(error)) = NFTMasterEvent::decode(&mut payload) {
                Some(error)
            } else {
                None
            }
        });

        assert_eq!(maybe_error.is_some(), error);
    }

    fn get_state(&self) -> NFTMasterState {
        self.read_state().expect("Unexpected invalid state.")
    }
}
