use base_io::{BaseAction, EquippableList, FixedPart, InitBase, Part, SlotPart};
use gstd::{prelude::*, BTreeMap};
use gtest::{Program, RunResult, System};
use resource_io::Resource;
use rmrk_io::*;
use types::primitives::{CollectionAndToken, PartId, ResourceId};
pub const USERS: &[u64] = &[10, 11, 12, 13];
pub const ZERO_ID: u64 = 0;
pub const PARENT_NFT_CONTRACT: u64 = 2;
pub const CHILD_NFT_CONTRACT: u64 = 1;

pub const BASE_ID: u64 = 3;
pub const CHILD_RESOURCE_ID: ResourceId = 150;
pub const PARENT_RESOURCE_ID: ResourceId = 151;

fn check_run_result_for_error(run_result: &RunResult, exp_error: &str) {
    assert!(run_result.main_failed());
    let error = String::from_utf8((run_result.log()[0].payload()).to_vec())
        .expect("Failed to decode error");
    if !error.contains(exp_error) {
        println!("Received panic {error:?}");
        println!("Expected panic {exp_error:?}");
        panic!("");
    }
}

pub trait RMRKToken {
    fn rmrk(sys: &System, resource_hash: Option<[u8; 32]>) -> Program;
    fn mint_to_root_owner(
        &self,
        user: u64,
        root_owner: u64,
        token_id: u64,
        exp_error: Option<&str>,
    );
    fn mint_to_nft(
        &self,
        user: u64,
        parent_id: u64,
        parent_token_id: u64,
        token_id: u64,
        exp_error: Option<&str>,
    );
    fn burn(&self, token_id: u64, user: u64, exp_error: Option<&str>);
    fn accept_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<&str>,
    );
    fn reject_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<&str>,
    );
    fn remove_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<&str>,
    );
    fn approve(&self, user: u64, to: u64, token_id: u64);
    fn transfer(&self, from: u64, to: u64, token_id: u64, exp_error: Option<&str>);
    fn transfer_to_nft(
        &self,
        from: u64,
        to: u64,
        token_id: u64,
        destination_id: u64,
        exp_error: Option<&str>,
    );
    // fn check_rmrk_owner(&self, token_id: u64, expected_token_id: Option<TokenId>, owner_id: u64);
    // fn check_balance(&self, user: u64, balance: u64);
    // fn check_pending_children(
    //     &self,
    //     parent_token_id: u64,
    //     expected_pending_children: BTreeSet<(CollectionId, TokenId)>,
    // );
    // fn check_accepted_children(
    //     &self,
    //     parent_token_id: u64,
    //     expected_accepted_children: BTreeSet<(CollectionId, TokenId)>,
    // );
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
        exp_error: Option<&str>,
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
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::MintToNft {
                parent_id: parent_id.into(),
                parent_token_id: parent_token_id.into(),
                token_id: token_id.into(),
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn mint_to_root_owner(
        &self,
        user: u64,
        root_owner: u64,
        token_id: u64,
        exp_error: Option<&str>,
    ) {
        let res = self.send(
            user,
            RMRKAction::MintToRootOwner {
                root_owner: root_owner.into(),
                token_id: token_id.into(),
            },
        );
        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::MintToRootOwner {
                root_owner: root_owner.into(),
                token_id: token_id.into(),
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn burn(&self, user: u64, token_id: u64, exp_error: Option<&str>) {
        let res = self.send(user, RMRKAction::Burn(token_id.into()));

        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::Transfer {
                to: ZERO_ID.into(),
                token_id: token_id.into(),
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn accept_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<&str>,
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
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::AcceptedChild {
                child_contract_id: child_contract_id.into(),
                child_token_id: child_token_id.into(),
                parent_token_id: parent_token_id.into(),
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn reject_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<&str>,
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
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::RejectedChild {
                child_contract_id: child_contract_id.into(),
                child_token_id: child_token_id.into(),
                parent_token_id: parent_token_id.into(),
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn remove_child(
        &self,
        user: u64,
        parent_token_id: u64,
        child_contract_id: u64,
        child_token_id: u64,
        exp_error: Option<&str>,
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
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::RemovedChild {
                child_contract_id: child_contract_id.into(),
                child_token_id: child_token_id.into(),
                parent_token_id: parent_token_id.into(),
            }
            .encode();
            assert!(res.contains(&(user, reply)));
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
        let reply = RMRKEvent::Approval {
            root_owner: user.into(),
            approved_account: to.into(),
            token_id: token_id.into(),
        }
        .encode();
        assert!(res.contains(&(user, reply)));
    }

    fn transfer(&self, from: u64, to: u64, token_id: u64, exp_error: Option<&str>) {
        let res = self.send(
            from,
            RMRKAction::Transfer {
                to: to.into(),
                token_id: token_id.into(),
            },
        );

        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::Transfer {
                to: to.into(),
                token_id: token_id.into(),
            }
            .encode();
            assert!(res.contains(&(from, reply)));
        }
    }

    fn transfer_to_nft(
        &self,
        from: u64,
        to: u64,
        token_id: u64,
        destination_id: u64,
        exp_error: Option<&str>,
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
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::TransferToNft {
                to: to.into(),
                token_id: token_id.into(),
                destination_id: destination_id.into(),
            }
            .encode();
            assert!(res.contains(&(from, reply)));
        }
    }

    // fn check_rmrk_owner(&self, token_id: u64, expected_token_id: Option<TokenId>, owner_id: u64) {
    //     let rmrk_owner: RMRKStateReply = self
    //         .meta_state(&RMRKState::Owner(token_id.into()))
    //         .expect("Meta_state failed");
    //     assert_eq!(
    //         rmrk_owner,
    //         RMRKStateReply::Owner {
    //             token_id: expected_token_id,
    //             owner_id: owner_id.into(),
    //         }
    //     );
    // }

    // fn check_balance(&self, account: u64, expected_balance: u64) {
    //     let balance: RMRKStateReply = self
    //         .meta_state(RMRKState::Balance(account.into()))
    //         .expect("Meta_state failed");
    //     assert_eq!(balance, RMRKStateReply::Balance(expected_balance.into()));
    // }

    // fn check_pending_children(
    //     &self,
    //     token_id: u64,
    //     expected_pending_children: BTreeSet<(CollectionId, TokenId)>,
    // ) {
    //     let pending_children: RMRKStateReply = self
    //         .meta_state(RMRKState::PendingChildren(token_id.into()))
    //         .expect("Meta_state failed");
    //     assert_eq!(
    //         pending_children,
    //         RMRKStateReply::PendingChildren(expected_pending_children),
    //     );
    // }

    // fn check_accepted_children(
    //     &self,
    //     parent_token_id: u64,
    //     expected_accepted_children: BTreeSet<(CollectionId, TokenId)>,
    // ) {
    //     let accepted_children: RMRKStateReply = self
    //         .meta_state(RMRKState::AcceptedChildren(parent_token_id.into()))
    //         .expect("Meta_state failed");
    //     assert_eq!(
    //         accepted_children,
    //         RMRKStateReply::AcceptedChildren(expected_accepted_children),
    //     );
    // }
    fn check_root_owner(&self, token_id: u64, root_owner: u64) {
        let res = self.send(10, RMRKAction::RootOwner(token_id.into()));
        assert!(res.contains(&(10, RMRKEvent::RootOwner(root_owner.into(),).encode())));
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

pub trait MultiResource {
    fn add_resource_entry(
        &self,
        user: u64,
        resource_id: ResourceId,
        resource: Resource,
        exp_error: Option<&str>,
    );
    fn add_resource(
        &self,
        user: u64,
        token_id: u64,
        resource_id: ResourceId,
        overwrite_id: ResourceId,
        exp_error: Option<&str>,
    );
    fn accept_resource(
        &self,
        user: u64,
        token_id: u64,
        resource_id: ResourceId,
        exp_error: Option<&str>,
    );
    fn reject_resource(
        &self,
        user: u64,
        token_id: u64,
        resource_id: ResourceId,
        exp_error: Option<&str>,
    );
    fn set_priority(&self, user: u64, token_id: u64, priorities: Vec<u8>, exp_error: Option<&str>);
    // fn check_pending_resources(
    //     &self,
    //     token_id: u64,
    //     expected_pending_resources: BTreeSet<ResourceId>,
    // );
    // fn check_active_resources(
    //     &self,
    //     token_id: u64,
    //     expected_pending_resources: BTreeSet<ResourceId>,
    // );
}

impl MultiResource for Program<'_> {
    fn add_resource_entry(
        &self,
        user: u64,
        resource_id: ResourceId,
        resource: Resource,
        exp_error: Option<&str>,
    ) {
        let res = self.send(
            user,
            RMRKAction::AddResourceEntry {
                resource_id,
                resource: resource.clone(),
            },
        );

        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::ResourceEntryAdded(resource).encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn add_resource(
        &self,
        user: u64,
        token_id: u64,
        resource_id: ResourceId,
        overwrite_id: ResourceId,
        exp_error: Option<&str>,
    ) {
        let res = self.send(
            user,
            RMRKAction::AddResource {
                token_id: token_id.into(),
                resource_id,
                overwrite_id,
            },
        );

        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::ResourceAdded {
                token_id: token_id.into(),
                resource_id,
                overwrite_id,
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn accept_resource(
        &self,
        user: u64,
        token_id: u64,
        resource_id: ResourceId,
        exp_error: Option<&str>,
    ) {
        let res = self.send(
            user,
            RMRKAction::AcceptResource {
                token_id: token_id.into(),
                resource_id,
            },
        );
        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::ResourceAccepted {
                token_id: token_id.into(),
                resource_id,
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn reject_resource(
        &self,
        user: u64,
        token_id: u64,
        resource_id: ResourceId,
        exp_error: Option<&str>,
    ) {
        let res = self.send(
            user,
            RMRKAction::RejectResource {
                token_id: token_id.into(),
                resource_id,
            },
        );
        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::ResourceRejected {
                token_id: token_id.into(),
                resource_id,
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    fn set_priority(&self, user: u64, token_id: u64, priorities: Vec<u8>, exp_error: Option<&str>) {
        let res = self.send(
            user,
            RMRKAction::SetPriority {
                token_id: token_id.into(),
                priorities: priorities.clone(),
            },
        );

        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::PrioritySet {
                token_id: token_id.into(),
                priorities,
            }
            .encode();
            assert!(res.contains(&(user, reply)));
        }
    }

    // fn check_pending_resources(
    //     &self,
    //     token_id: u64,
    //     expected_pending_resources: BTreeSet<ResourceId>,
    // ) {
    //     let pending_resources: RMRKStateReply = self
    //         .meta_state(RMRKState::PendingResources(token_id.into()))
    //         .expect("Meta_state failed");
    //     assert_eq!(
    //         pending_resources,
    //         RMRKStateReply::PendingResources(expected_pending_resources)
    //     );
    // }

    // fn check_active_resources(
    //     &self,
    //     token_id: u64,
    //     expected_active_resources: BTreeSet<ResourceId>,
    // ) {
    //     let active_resources: RMRKStateReply = self
    //         .meta_state(RMRKState::ActiveResources(token_id.into()))
    //         .expect("Meta_state failed");
    //     assert_eq!(
    //         active_resources,
    //         RMRKStateReply::ActiveResources(expected_active_resources)
    //     );
    // }
}
pub fn init_base(sys: &System) {
    let base = Program::from_file(sys, "../target/wasm32-unknown-unknown/debug/rmrk_base.wasm");
    let res = base.send(
        USERS[0],
        InitBase {
            base_type: "svg".to_string(),
            symbol: "".to_string(),
        },
    );
    assert!(res.log().is_empty());

    let mut parts: BTreeMap<PartId, Part> = BTreeMap::new();
    // setup base
    let fixed_part_body_id = 100;
    let fixed_part_body = FixedPart {
        z: Some(0),
        src: "body-1".to_string(),
    };
    parts.insert(fixed_part_body_id, Part::Fixed(fixed_part_body));

    // Slot part left hand can equip items from collections 0 or 1
    let slot_part_left_hand_id = 400;
    let slot_part_left_hand = SlotPart {
        z: Some(0),
        src: "left-hand".to_string(),
        equippable: EquippableList::All,
    };
    parts.insert(slot_part_left_hand_id, Part::Slot(slot_part_left_hand));
    // add parts to base
    assert!(!base
        .send(USERS[0], BaseAction::AddParts(parts))
        .main_failed());
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

pub fn mint_token_and_add_resource(
    rmrk: &Program,
    token_id: u64,
    resource_id: ResourceId,
    resource: Resource,
) {
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);
    rmrk.add_resource_entry(USERS[0], resource_id, resource, None);
}

pub fn mint_token_and_add_resource_to_token(
    rmrk: &Program,
    token_id: u64,
    resource_id: ResourceId,
    resource: Resource,
    overwrite_id: ResourceId,
) {
    mint_token_and_add_resource(rmrk, token_id, resource_id, resource);
    rmrk.add_resource(USERS[0], token_id, resource_id, overwrite_id, None);
}

pub fn mint_token_and_add_resource_to_token_with_acceptance(
    rmrk: &Program,
    user: u64,
    token_id: u64,
    resource_id: ResourceId,
    resource: Resource,
) {
    mint_token_and_add_resource_to_token(rmrk, token_id, resource_id, resource, 0);
    rmrk.accept_resource(user, token_id, resource_id, None);
}

pub trait Equippable {
    fn equip(
        &self,
        token_id: u64,
        resource_id: ResourceId,
        equippable: CollectionAndToken,
        equippable_resource_id: ResourceId,
        exp_error: Option<&str>,
    );
}

impl Equippable for Program<'_> {
    fn equip(
        &self,
        token_id: u64,
        resource_id: ResourceId,
        equippable: CollectionAndToken,
        equippable_resource_id: ResourceId,
        exp_error: Option<&str>,
    ) {
        let res = self.send(
            USERS[0],
            RMRKAction::Equip {
                token_id: token_id.into(),
                resource_id,
                equippable,
                equippable_resource_id,
            },
        );

        if let Some(exp_error) = exp_error {
            check_run_result_for_error(&res, exp_error);
        } else {
            let reply = RMRKEvent::TokenEquipped {
                token_id: token_id.into(),
                resource_id,
                slot_id: 400,
                equippable,
            }
            .encode();
            assert!(res.contains(&(USERS[0], reply)));
        }
    }
}
