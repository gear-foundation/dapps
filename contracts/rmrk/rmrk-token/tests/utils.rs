use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use hashbrown::HashSet;
use primitive_types::U256;
use rmrk_io::*;
use rmrk_state::WASM_BINARY;

use types::primitives::{CollectionId, TokenId};
pub const USERS: &[u64] = &[10, 11, 12, 13];
pub const ZERO_ID: u64 = 0;
pub const PARENT_NFT_CONTRACT: u64 = 2;
pub const CHILD_NFT_CONTRACT: u64 = 1;

//pub const CATALOG_ID: u64 = 3;

pub trait RMRKToken {
    fn rmrk(sys: &System, resource_hash: Option<[u8; 32]>) -> Program;
    fn mint_to_root_owner(
        &self,
        user: u64,
        root_owner: u64,
        token_id: u64,
        exp_error: Option<RMRKError>,
    );
    fn mint_to_nft(
        &self,
        user: u64,
        parent_id: u64,
        parent_token_id: u64,
        token_id: u64,
        exp_error: Option<RMRKError>,
    );
    fn burn(&self, token_id: u64, user: u64, exp_error: Option<RMRKError>);
    fn accept_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<RMRKError>,
    );
    fn reject_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<RMRKError>,
    );
    fn remove_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<RMRKError>,
    );
    fn approve(&self, user: u64, to: u64, token_id: u64);
    fn transfer(&self, from: u64, to: u64, token_id: u64, exp_error: Option<RMRKError>);
    fn transfer_to_nft(
        &self,
        from: u64,
        to: u64,
        token_id: u64,
        destination_id: u64,
        exp_error: Option<RMRKError>,
    );
    fn check_rmrk_owner(&self, token_id: u64, expected_token_id: Option<TokenId>, owner_id: u64);
    fn check_balance(&self, user: ActorId, balance: U256);
    fn check_pending_children(
        &self,
        token_id: u64,
        expected_pending_children: HashSet<(CollectionId, TokenId)>,
    );
    fn check_accepted_children(
        &self,
        token_id: u64,
        expected_accepted_children: HashSet<(CollectionId, TokenId)>,
    );
    fn check_root_owner(&self, token_id: u64, root_owner: u64);
}

impl RMRKToken for Program<'_> {
    fn rmrk(sys: &System, resource_hash: Option<[u8; 32]>) -> Program {
        let rmrk = Program::current(sys);
        let res = rmrk.send(
            USERS[0],
            InitRMRK {
                name: "RMRKToken".to_string(),
                symbol: "RMRKSymbol".to_string(),
                resource_hash,
                resource_name: "ResourceName".to_string(),
            },
        );
        assert!(!res.main_failed());
        rmrk
    }

    fn mint_to_nft(
        &self,
        user: u64,
        parent_id: u64,
        parent_token_id: u64,
        token_id: u64,
        exp_error: Option<RMRKError>,
    ) {
        let res = self.send(
            user,
            RMRKAction::MintToNft {
                parent_id: parent_id.into(),
                parent_token_id: parent_token_id.into(),
                token_id: token_id.into(),
            },
        );
        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            assert!(res.contains(&(user, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::MintedToNft);
            assert!(res.contains(&(user, reply.encode())));
        }
    }

    fn mint_to_root_owner(
        &self,
        user: u64,
        root_owner: u64,
        token_id: u64,
        exp_error: Option<RMRKError>,
    ) {
        let res = self.send(
            user,
            RMRKAction::MintToRootOwner {
                root_owner: root_owner.into(),
                token_id: token_id.into(),
            },
        );
        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            assert!(res.contains(&(user, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::MintedToRootOwner);
            assert!(res.contains(&(user, reply.encode())));
        }
    }

    fn burn(&self, user: u64, token_id: u64, exp_error: Option<RMRKError>) {
        let res = self.send(user, RMRKAction::Burn(token_id.into()));

        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            let decoded_log = res.decoded_log::<Result<RMRKReply, RMRKError>>();
            println!("DECODED LOG {:?}", decoded_log);
            assert!(res.contains(&(user, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::Burnt);
            let decoded_log = res.decoded_log::<Result<RMRKReply, RMRKError>>();
            println!("DECODED LOG {:?}", decoded_log);
            assert!(res.contains(&(user, reply.encode())));
        }
    }

    fn accept_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<RMRKError>,
    ) {
        let res = self.send(
            user,
            RMRKAction::AcceptChild {
                parent_token_id: parent_token_id.into(),
                child_contract_id: child_contract_id.into(),
                child_token_id: child_token_id.into(),
            },
        );

        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            let decoded_log = res.decoded_log::<Result<RMRKReply, RMRKError>>();
            println!("DECODED LOG {:?}", decoded_log);
            assert!(res.contains(&(user, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildAccepted);
            assert!(res.contains(&(user, reply.encode())));
        }
    }

    fn reject_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<RMRKError>,
    ) {
        let res = self.send(
            user,
            RMRKAction::RejectChild {
                parent_token_id: parent_token_id.into(),
                child_contract_id: child_contract_id.into(),
                child_token_id: child_token_id.into(),
            },
        );

        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            let decoded_log = res.decoded_log::<Result<RMRKReply, RMRKError>>();
            println!("DECODED LOG {:?}", decoded_log);
            assert!(res.contains(&(user, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildRejected);
            assert!(res.contains(&(user, reply.encode())));
        }
    }

    fn remove_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<RMRKError>,
    ) {
        let res = self.send(
            user,
            RMRKAction::RemoveChild {
                parent_token_id: parent_token_id.into(),
                child_contract_id: child_contract_id.into(),
                child_token_id: child_token_id.into(),
            },
        );

        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            assert!(res.contains(&(user, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildRemoved);
            assert!(res.contains(&(user, reply.encode())));
        }
    }

    fn approve(&self, user: u64, to: u64, token_id: u64) {
        let res = self.send(
            user,
            RMRKAction::Approve {
                to: to.into(),
                token_id: token_id.into(),
            },
        );
        let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::Approved);
        assert!(res.contains(&(user, reply.encode())));
    }

    fn transfer(&self, from: u64, to: u64, token_id: u64, exp_error: Option<RMRKError>) {
        let res = self.send(
            from,
            RMRKAction::Transfer {
                to: to.into(),
                token_id: token_id.into(),
            },
        );

        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            assert!(res.contains(&(from, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::Transferred);
            assert!(res.contains(&(from, reply.encode())));
        }
    }

    fn transfer_to_nft(
        &self,
        from: u64,
        to: u64,
        token_id: u64,
        destination_id: u64,
        exp_error: Option<RMRKError>,
    ) {
        let res = self.send(
            from,
            RMRKAction::TransferToNft {
                to: to.into(),
                token_id: token_id.into(),
                destination_id: destination_id.into(),
            },
        );

        if let Some(exp_error) = exp_error {
            let error: Result<RMRKReply, RMRKError> = Err(exp_error);
            let decoded_log = res.decoded_log::<Result<RMRKReply, RMRKError>>();
            println!(" ERROR LOG {:?}", decoded_log);
            assert!(res.contains(&(from, error.encode())));
        } else {
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::TransferredToNft);
            let decoded_log = res.decoded_log::<Result<RMRKReply, RMRKError>>();
            println!(" DECODED LOG {:?}", decoded_log);
            assert!(res.contains(&(from, reply.encode())));
        }
    }

    fn check_rmrk_owner(&self, token_id: u64, expected_token_id: Option<TokenId>, owner_id: u64) {
        let rmrk_owner: RMRKOwner = self
            .read_state_using_wasm(
                "rmrk_owner",
                WASM_BINARY.into(),
                Some(TokenId::from(token_id)),
            )
            .expect("Failed to read state");
        assert_eq!(
            rmrk_owner,
            RMRKOwner {
                token_id: expected_token_id,
                owner_id: owner_id.into(),
            }
        );
    }

    fn check_balance(&self, account: ActorId, expected_balance: U256) {
        let balance: U256 = self
            .read_state_using_wasm("balance", WASM_BINARY.into(), Some(account))
            .expect("Failed to read state");
        assert_eq!(balance, expected_balance);
    }

    fn check_pending_children(
        &self,
        token_id: u64,
        expected_pending_children: HashSet<(CollectionId, TokenId)>,
    ) {
        let pending_children: Vec<(CollectionId, TokenId)> = self
            .read_state_using_wasm(
                "pending_children",
                WASM_BINARY.into(),
                Some(TokenId::from(token_id)),
            )
            .expect("Failed to read state");
        let pending_children: HashSet<(CollectionId, TokenId)> =
            HashSet::from_iter(pending_children);
        assert_eq!(pending_children, expected_pending_children,);
    }

    fn check_accepted_children(
        &self,
        token_id: u64,
        expected_accepted_children: HashSet<(CollectionId, TokenId)>,
    ) {
        let accepted_children: Vec<(CollectionId, TokenId)> = self
            .read_state_using_wasm(
                "accepted_children",
                WASM_BINARY.into(),
                Some(TokenId::from(token_id)),
            )
            .expect("Failed to read state");
        let accepted_children: HashSet<(CollectionId, TokenId)> =
            HashSet::from_iter(accepted_children);
        assert_eq!(accepted_children, expected_accepted_children);
    }
    fn check_root_owner(&self, token_id: u64, root_owner: u64) {
        let res = self.send(10, RMRKAction::RootOwner(token_id.into()));
        let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::RootOwner(root_owner.into()));
        assert!(res.contains(&(10, reply.encode())));
    }
}

pub fn mint_parent_and_child(
    rmrk_child: &Program,
    rmrk_parent: &Program,
    child_token_id: u64,
    parent_token_id: u64,
) {
    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], parent_token_id, None);

    // mint RMRK child token to RMRK parent token
    rmrk_child.mint_to_nft(
        USERS[3],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_token_id,
        None,
    );
}

pub fn mint_parent_and_child_with_acceptance(
    rmrk_child: &Program,
    rmrk_parent: &Program,
    child_token_id: u64,
    parent_token_id: u64,
) {
    mint_parent_and_child(rmrk_child, rmrk_parent, child_token_id, parent_token_id);
    // accept child
    rmrk_parent.accept_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );
}

// ownership chain is  USERS[0] > parent_token_id > child_token_id > grand_token_id
pub fn rmrk_chain(
    rmrk_grand: &Program,
    rmrk_child: &Program,
    rmrk_parent: &Program,
    grand_token_id: u64,
    child_token_id: u64,
    parent_token_id: u64,
) {
    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], parent_token_id, None);

    // mint child_token_id to parent_token_id
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_token_id,
        None,
    );
    // accept child
    rmrk_parent.accept_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );
    // mint grand_token_id to child_token_id
    rmrk_grand.mint_to_nft(
        USERS[1],
        CHILD_NFT_CONTRACT,
        child_token_id,
        grand_token_id,
        None,
    );

    // accept child
    rmrk_child.accept_child(USERS[0], child_token_id, 3, grand_token_id, None);
}
