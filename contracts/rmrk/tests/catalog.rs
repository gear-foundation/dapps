use gstd::{collections::BTreeMap, prelude::*};
use gtest::{Program, System};
use rmrk_catalog_io::*;
pub const ADMIN: u64 = 10;

pub fn init_catalog(sys: &System, admin: u64) {
    sys.init_logger();
    sys.mint_to(admin, 100_000_000_000_000);
    let catalog = Program::from_file(
        sys,
        "../target/wasm32-gear/release/rmrk_catalog.opt.wasm",
    );
    let mid = catalog.send(
        admin,
        InitCatalog {
            catalog_type: "svg".to_string(),
            symbol: "BaseSymbol".to_string(),
        },
    );
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

#[test]
fn add_parts() {
    let system = System::new();
    init_catalog(&system, ADMIN);
    let catalog = system.get_program(1).unwrap();

    // Add fixed part
    let fixed_part_data = Part::Fixed(FixedPart {
        z: Some(0),
        metadata_uri: String::from("src"),
    });
    let part_id = 1;

    let added_part = BTreeMap::from([(part_id, fixed_part_data.clone())]);

    catalog.send(ADMIN, CatalogAction::AddParts(added_part.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> =
        Ok(CatalogReply::PartsAdded(added_part));
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // check that fixed part is in the state
    let state: CatalogState = catalog.read_state(0).expect("Failed to decode the state");
    let fixed_part_in_state = state
        .parts
        .iter()
        .any(|part| part == &(part_id, fixed_part_data.clone()));
    assert!(fixed_part_in_state);

    // Add slot part
    let slot_part_data = Part::Slot(SlotPart {
        equippable: vec![],
        z: Some(0),
        metadata_uri: String::from("src"),
    });
    let part_id = 2;

    let added_part = BTreeMap::from([(part_id, slot_part_data.clone())]);
    catalog.send(ADMIN, CatalogAction::AddParts(added_part.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> =
        Ok(CatalogReply::PartsAdded(added_part));
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Check that slot part is in the state
    let state: CatalogState = catalog.read_state(0).expect("Failed to decode the state");

    let slot_part_in_state = state
        .parts
        .iter()
        .any(|part| part == &(part_id, slot_part_data.clone()));
    assert!(slot_part_in_state);

    // Add part list
    let slot_part_id = 10;
    let fixed_part_id_1 = 20;
    let fixed_part_id_2 = 21;

    let fixed_part_data_1 = Part::Fixed(FixedPart {
        z: Some(1),
        metadata_uri: String::from("src1"),
    });
    let fixed_part_data_2 = Part::Fixed(FixedPart {
        z: Some(1),
        metadata_uri: String::from("src2"),
    });
    let slot_part_data = Part::Slot(SlotPart {
        equippable: vec![],
        z: Some(2),
        metadata_uri: String::from("src3"),
    });
    let mut parts = BTreeMap::new();
    parts.insert(slot_part_id, slot_part_data.clone());
    parts.insert(fixed_part_id_1, fixed_part_data_1.clone());
    parts.insert(fixed_part_id_2, fixed_part_data_2.clone());

    catalog.send(ADMIN, CatalogAction::AddParts(parts.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::PartsAdded(parts));
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    let state: CatalogState = catalog.read_state(0).expect("Failed to decode the state");

    // check that fixed part_1 is in the state
    let fixed_part_1_in_state = state
        .parts
        .iter()
        .any(|part| part == &(fixed_part_id_1, fixed_part_data_1.clone()));
    assert!(fixed_part_1_in_state);

    // check that fixed part_2 is in the state
    let fixed_part_2_in_state = state
        .parts
        .iter()
        .any(|part| part == &(fixed_part_id_2, fixed_part_data_2.clone()));
    assert!(fixed_part_2_in_state);

    // check that slot part_1 is in the state
    let slot_part_in_state = state
        .parts
        .iter()
        .any(|part| part == &(slot_part_id, slot_part_data.clone()));
    assert!(slot_part_in_state);

    // Remove parts
    let removed_parts = vec![fixed_part_id_1, slot_part_id];
    catalog.send(ADMIN, CatalogAction::RemoveParts(removed_parts.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> =
        Ok(CatalogReply::PartsRemoved(removed_parts));
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // check that fixed part_1 is NOT in the state
    let state: CatalogState = catalog.read_state(0).expect("Failed to decode the state");
    let fixed_part_1_in_state = state
        .parts
        .iter()
        .any(|part| part == &(fixed_part_id_1, fixed_part_data_1.clone()));
    assert!(!fixed_part_1_in_state);

    // check that slot part_1 is NOT in the state
    let slot_part_in_state = state
        .parts
        .iter()
        .any(|part| part == &(slot_part_id, slot_part_data.clone()));
    assert!(!slot_part_in_state);

    // Zero length array of parts
    catalog.send(ADMIN, CatalogAction::RemoveParts(vec![]));
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::ZeroLengthPassed);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot remove non-existing part
    catalog.send(ADMIN, CatalogAction::RemoveParts(vec![fixed_part_id_1]));
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::PartDoesNotExist);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));
}

#[test]
fn add_parts_error_cases() {
    let system = System::new();
    init_catalog(&system, ADMIN);
    let catalog = system.get_program(1).unwrap();

    let fixed_part_data = Part::Fixed(FixedPart {
        z: Some(0),
        metadata_uri: String::from("src"),
    });
    let part_id = 0;

    let added_part = BTreeMap::from([(part_id, fixed_part_data.clone())]);

    // Cannot add part with zero id
    catalog.send(ADMIN, CatalogAction::AddParts(added_part));
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::PartIdCantBeZero);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // check that fixed part is in the state
    let state: CatalogState = catalog.read_state(0).expect("Failed to decode the state");
    assert_eq!(state.parts, vec![]);

    // Add part
    let part_id = 1;

    let added_part = BTreeMap::from([(part_id, fixed_part_data.clone())]);

    catalog.send(ADMIN, CatalogAction::AddParts(added_part.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> =
        Ok(CatalogReply::PartsAdded(added_part));
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot add part with already existing id
    let added_part = BTreeMap::from([(part_id, fixed_part_data)]);
    catalog.send(ADMIN, CatalogAction::AddParts(added_part));
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::PartAlreadyExists);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Zero length BTreeMap
    catalog.send(ADMIN, CatalogAction::AddParts(BTreeMap::new()));
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::ZeroLengthPassed);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));
}

#[test]
fn equippable() {
    let system = System::new();
    init_catalog(&system, ADMIN);
    let catalog = system.get_program(1).unwrap();

    // Add fixed part
    let fixed_part_id = 1;
    let fixed_part_data = Part::Fixed(FixedPart {
        z: Some(0),
        metadata_uri: String::from("src"),
    });

    let slot_part_id_1 = 2;
    let slot_part_data_1 = Part::Slot(SlotPart {
        equippable: vec![100.into()],
        z: Some(0),
        metadata_uri: String::from("src"),
    });

    let slot_part_id_2 = 3;
    let slot_part_data_2 = Part::Slot(SlotPart {
        equippable: vec![],
        z: Some(0),
        metadata_uri: String::from("src"),
    });

    let mut parts = BTreeMap::new();
    parts.insert(fixed_part_id, fixed_part_data);
    parts.insert(slot_part_id_1, slot_part_data_1);
    parts.insert(slot_part_id_2, slot_part_data_2);

    catalog.send(ADMIN, CatalogAction::AddParts(parts.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::PartsAdded(parts));
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Is not equippable if address was not added
    catalog.send(
        ADMIN,
        CatalogAction::CheckEquippable {
            part_id: slot_part_id_2,
            collection_id: 100.into(),
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::NotInEquippableList);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Is equippable if added in the part definition
    catalog.send(
        ADMIN,
        CatalogAction::CheckEquippable {
            part_id: slot_part_id_1,
            collection_id: 100.into(),
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::InEquippableList);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Is equippable if added afterward
    catalog.send(
        ADMIN,
        CatalogAction::AddEquippableAddresses {
            part_id: slot_part_id_2,
            collection_ids: vec![100.into()],
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::EquippablesAdded {
        part_id: slot_part_id_2,
        collection_ids: vec![100.into()],
    });
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));
    catalog.send(
        ADMIN,
        CatalogAction::CheckEquippable {
            part_id: slot_part_id_2,
            collection_id: 100.into(),
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::InEquippableList);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Is equippable if set to all
    catalog.send(
        ADMIN,
        CatalogAction::SetEquippableToAll {
            part_id: slot_part_id_1,
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::EquippableToAllSet);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    catalog.send(
        ADMIN,
        CatalogAction::CheckEquippable {
            part_id: slot_part_id_1,
            collection_id: 200.into(),
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::InEquippableList);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Can reset equippable addresses
    // Reset the slot that is equippable to all
    catalog.send(
        ADMIN,
        CatalogAction::ResetEquippableAddress {
            part_id: slot_part_id_1,
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> =
        Ok(CatalogReply::EqippableAddressesReset);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    catalog.send(
        ADMIN,
        CatalogAction::CheckEquippable {
            part_id: slot_part_id_1,
            collection_id: 200.into(),
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::NotInEquippableList);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Reset the slot that is equippable to indixated addresses
    catalog.send(
        ADMIN,
        CatalogAction::ResetEquippableAddress {
            part_id: slot_part_id_2,
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> =
        Ok(CatalogReply::EqippableAddressesReset);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    catalog.send(
        ADMIN,
        CatalogAction::CheckEquippable {
            part_id: slot_part_id_2,
            collection_id: 100.into(),
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::NotInEquippableList);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot add equippable addresses for non existing part
    catalog.send(
        ADMIN,
        CatalogAction::AddEquippableAddresses {
            part_id: 100,
            collection_ids: vec![100.into()],
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::PartDoesNotExist);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot add empty list of equippable addresses
    catalog.send(
        ADMIN,
        CatalogAction::AddEquippableAddresses {
            part_id: slot_part_id_1,
            collection_ids: vec![],
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::ZeroLengthPassed);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot add equippable addresses to non slot part
    catalog.send(
        ADMIN,
        CatalogAction::AddEquippableAddresses {
            part_id: fixed_part_id,
            collection_ids: vec![200.into()],
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::WrongPartFormat);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot reset equippable for non existing part
    catalog.send(
        ADMIN,
        CatalogAction::ResetEquippableAddress { part_id: 1000 },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::PartDoesNotExist);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));

    // Cannot reset equippable for fixed part
    catalog.send(
        ADMIN,
        CatalogAction::ResetEquippableAddress {
            part_id: fixed_part_id,
        },
    );
    let expected_reply: Result<CatalogReply, CatalogError> = Err(CatalogError::WrongPartFormat);
    let result = system.run_next_block();
    assert!(result.contains(&(ADMIN, expected_reply.encode())));
}
