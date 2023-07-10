use catalog_io::*;
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use rmrk_io::*;
use rmrk_state::WASM_BINARY;
use types::primitives::{PartId, TokenId};
const CATALOG_ID: u64 = 100;
const PATH_TO_CATALOG: &str = "../target/wasm32-unknown-unknown/release/rmrk_catalog.opt.wasm";
const ADMIN: u64 = 200;
const KANARIA_ID: u64 = 10;
const GEM_ID: u64 = 11;

pub fn setup_catalog(system: &System) {
    let mut parts = BTreeMap::new();

    let catalog = Program::from_file_with_id(system, CATALOG_ID, PATH_TO_CATALOG);
    let res = catalog.send(
        ADMIN,
        InitCatalog {
            catalog_type: "svg".to_string(),
            symbol: "CatalogSymbol".to_string(),
        },
    );
    assert!(!res.main_failed());

    let part_id_for_back_1 = 1;
    let part_for_back_1 = Part::Fixed(FixedPart {
        z: Some(0),
        metadata_uri: String::from("ipfs://backgrounds/1.svg"),
    });
    parts.insert(part_id_for_back_1, part_for_back_1);

    let part_id_for_back_2 = 2;
    let part_for_back_2 = Part::Fixed(FixedPart {
        z: Some(0),
        metadata_uri: String::from("ipfs://backgrounds/2.svg"),
    });
    parts.insert(part_id_for_back_2, part_for_back_2);

    let part_id_for_head_1 = 3;
    let part_for_head_1 = Part::Fixed(FixedPart {
        z: Some(3),
        metadata_uri: String::from("ipfs://heads/1.svg"),
    });
    parts.insert(part_id_for_head_1, part_for_head_1);

    let part_id_for_head_2 = 4;
    let part_for_head_2 = Part::Fixed(FixedPart {
        z: Some(3),
        metadata_uri: String::from("ipfs://heads/2.svg"),
    });
    parts.insert(part_id_for_head_2, part_for_head_2);

    let part_id_for_body_1 = 5;
    let part_for_body_1 = Part::Fixed(FixedPart {
        z: Some(2),
        metadata_uri: String::from("ipfs://body/1.svg"),
    });
    parts.insert(part_id_for_body_1, part_for_body_1);

    let part_id_for_body_2 = 6;
    let part_for_body_2 = Part::Fixed(FixedPart {
        z: Some(2),
        metadata_uri: String::from("ipfs://body/2.svg"),
    });
    parts.insert(part_id_for_body_2, part_for_body_2);

    let part_id_for_wings_1 = 7;
    let part_for_wings_1 = Part::Fixed(FixedPart {
        z: Some(4),
        metadata_uri: String::from("ipfs://wings/1.svg"),
    });
    parts.insert(part_id_for_wings_1, part_for_wings_1);

    let part_id_for_wings_2 = 8;
    let part_for_wings_2 = Part::Fixed(FixedPart {
        z: Some(4),
        metadata_uri: String::from("ipfs://wings/2.svg"),
    });
    parts.insert(part_id_for_wings_2, part_for_wings_2);

    let part_id_for_gem_slot_1 = 9;
    let part_for_gem_slot_1 = Part::Slot(SlotPart {
        equippable: vec![GEM_ID.into()],
        z: Some(4),
        metadata_uri: String::from(""),
    });
    parts.insert(part_id_for_gem_slot_1, part_for_gem_slot_1);

    let part_id_for_gem_slot_2 = 10;
    let part_for_gem_slot_2 = Part::Slot(SlotPart {
        equippable: vec![GEM_ID.into()],
        z: Some(4),
        metadata_uri: String::from(""),
    });
    parts.insert(part_id_for_gem_slot_2, part_for_gem_slot_2);

    let part_id_for_gem_slot_3 = 11;
    let part_for_gem_slot_3 = Part::Slot(SlotPart {
        equippable: vec![GEM_ID.into()],
        z: Some(4),
        metadata_uri: String::from(""),
    });
    parts.insert(part_id_for_gem_slot_3, part_for_gem_slot_3);

    let result = catalog.send(ADMIN, CatalogAction::AddParts(parts.clone()));
    let expected_reply: Result<CatalogReply, CatalogError> = Ok(CatalogReply::PartsAdded(parts));
    assert!(result.contains(&(ADMIN, expected_reply.encode())));
}

pub fn mint_tokens(system: &System) {
    let kanaria = Program::current_with_id(system, KANARIA_ID);

    let res = kanaria.send(
        ADMIN,
        InitRMRK {
            name: "Kanaria".to_string(),
            symbol: "KAN".to_string(),
            resource_hash: None,
            resource_name: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    let gem = Program::current_with_id(system, GEM_ID);
    let res = gem.send(
        ADMIN,
        InitRMRK {
            name: "Gem".to_string(),
            symbol: "GEM".to_string(),
            resource_hash: None,
            resource_name: "".to_string(),
        },
    );
    assert!(!res.main_failed());

    // mint 5 birds
    for token_id in 1..6 {
        let res = kanaria.send(
            ADMIN,
            RMRKAction::MintToRootOwner {
                root_owner: ADMIN.into(),
                token_id: token_id.into(),
            },
        );
        let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::MintedToRootOwner);
        assert!(res.contains(&(ADMIN, reply.encode())));
    }

    // Mint 3 gems into each kanaria
    let mut gem_token_id = 1;
    for token_id in 1..6 {
        for _i in 1..4 {
            let res = gem.send(
                ADMIN,
                RMRKAction::MintToNft {
                    parent_id: KANARIA_ID.into(),
                    parent_token_id: token_id.into(),
                    token_id: gem_token_id.into(),
                },
            );
            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::MintedToNft);
            assert!(res.contains(&(ADMIN, reply.encode())));

            let res = kanaria.send(
                ADMIN,
                RMRKAction::AcceptChild {
                    parent_token_id: token_id.into(),
                    child_contract_id: GEM_ID.into(),
                    child_token_id: gem_token_id.into(),
                },
            );

            let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildAccepted);
            assert!(res.contains(&(ADMIN, reply.encode())));
            gem_token_id += 1;
        }
    }
}

pub fn add_kanaria_assets(system: &System) {
    let kanaria = system.get_program(KANARIA_ID);
    let default_asset_id = 1;
    let composed_asset_id = 2;

    add_equippable_asset_entry(
        &kanaria,
        0,
        None,
        String::from("ipfs://default.png"),
        vec![],
        default_asset_id,
    );

    add_equippable_asset_entry(
        &kanaria,
        0,
        Some(CATALOG_ID.into()),
        String::from("ipfs://meta1.json"),
        vec![1, 3, 5, 7, 9, 10, 11],
        composed_asset_id,
    );

    let token_id: TokenId = 1.into();

    add_asset_to_token(&kanaria, token_id, default_asset_id, 0);
    add_asset_to_token(&kanaria, token_id, composed_asset_id, 0);

    accept_asset(&kanaria, token_id, default_asset_id);
    accept_asset(&kanaria, token_id, composed_asset_id);
}

pub fn add_gem_assets(system: &System) {
    let gem = system.get_program(GEM_ID);

    // These refIds are used from the child's perspective, to group assets that can be equipped into a parent
    // With it, we avoid the need to do set it asset by asset
    let equippable_ref_id_left_gem = 1;
    let equippable_ref_id_mid_gem = 2;
    let equippable_ref_id_right_gem = 3;

    add_equippable_asset_entry(
        &gem,
        0,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeA/full.svg"),
        vec![],
        1,
    );

    add_equippable_asset_entry(
        &gem,
        equippable_ref_id_left_gem,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeA/left.svg"),
        vec![],
        2,
    );

    add_equippable_asset_entry(
        &gem,
        equippable_ref_id_mid_gem,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeA/mid.svg"),
        vec![],
        3,
    );

    add_equippable_asset_entry(
        &gem,
        equippable_ref_id_right_gem,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeA/right.svg"),
        vec![],
        4,
    );

    add_equippable_asset_entry(
        &gem,
        0,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeB/full.svg"),
        vec![],
        5,
    );

    add_equippable_asset_entry(
        &gem,
        equippable_ref_id_left_gem,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeB/left.svg"),
        vec![],
        6,
    );

    add_equippable_asset_entry(
        &gem,
        equippable_ref_id_mid_gem,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeB/mid.svg"),
        vec![],
        7,
    );

    add_equippable_asset_entry(
        &gem,
        equippable_ref_id_right_gem,
        Some(CATALOG_ID.into()),
        String::from("ipfs://gems/typeB/right.svg"),
        vec![],
        8,
    );

    // 9, 10 and 11 are the slot part ids for the gems, defined on the catalog.
    // e.g. Any asset on gem, which sets its equippableRefId to equippableRefIdLeftGem
    // will be considered a valid equip into any kanaria on slot 9 (left gem).
    set_valid_parent_for_equippable_group(&gem, equippable_ref_id_left_gem, 9, KANARIA_ID.into());
    set_valid_parent_for_equippable_group(&gem, equippable_ref_id_mid_gem, 10, KANARIA_ID.into());
    set_valid_parent_for_equippable_group(&gem, equippable_ref_id_right_gem, 11, KANARIA_ID.into());

    // We add assets of type A to gem 1 and 2, and type B to gem 3. Both are nested into the first kanaria
    // This means gems 1 and 2 will have the same asset, which is totally valid.

    add_asset_to_token(&gem, 1.into(), 1, 0);
    add_asset_to_token(&gem, 1.into(), 2, 0);
    add_asset_to_token(&gem, 1.into(), 3, 0);
    add_asset_to_token(&gem, 1.into(), 4, 0);

    add_asset_to_token(&gem, 2.into(), 1, 0);
    add_asset_to_token(&gem, 2.into(), 2, 0);
    add_asset_to_token(&gem, 2.into(), 3, 0);
    add_asset_to_token(&gem, 2.into(), 4, 0);

    add_asset_to_token(&gem, 3.into(), 5, 0);
    add_asset_to_token(&gem, 3.into(), 6, 0);
    add_asset_to_token(&gem, 3.into(), 7, 0);
    add_asset_to_token(&gem, 3.into(), 8, 0);

    accept_asset(&gem, 1.into(), 1);
    accept_asset(&gem, 1.into(), 2);
    accept_asset(&gem, 1.into(), 3);
    accept_asset(&gem, 1.into(), 4);

    accept_asset(&gem, 2.into(), 1);
    accept_asset(&gem, 2.into(), 2);
    accept_asset(&gem, 2.into(), 3);
    accept_asset(&gem, 2.into(), 4);

    accept_asset(&gem, 3.into(), 5);
    accept_asset(&gem, 3.into(), 6);
    accept_asset(&gem, 3.into(), 7);
    accept_asset(&gem, 3.into(), 8);
}

pub fn equip_gems(system: &System) {
    let kanaria = system.get_program(KANARIA_ID);

    let result = kanaria.send(
        ADMIN,
        RMRKAction::Equip {
            token_id: 1.into(),       // Kanaria 1
            child_token_id: 1.into(), // Gem 1
            child_id: GEM_ID.into(),
            asset_id: 2,       // Asset for the kanaria which is composable
            slot_part_id: 9,   // left gem slot
            child_asset_id: 2, // Asset id for child meant for the left gem
        },
    );
    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildAssetEquipped);
    assert!(result.contains(&(ADMIN, reply.encode())));

    let result = kanaria.send(
        ADMIN,
        RMRKAction::Equip {
            token_id: 1.into(),       // Kanaria 1
            child_token_id: 2.into(), // Gem 2
            child_id: GEM_ID.into(),
            asset_id: 2,       // Asset for the kanaria which is composable
            slot_part_id: 10,  // mid gem slot
            child_asset_id: 3, // Asset id for child meant for the mid gem
        },
    );
    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildAssetEquipped);
    assert!(result.contains(&(ADMIN, reply.encode())));

    let result = kanaria.send(
        ADMIN,
        RMRKAction::Equip {
            token_id: 1.into(),       // Kanaria 1
            child_token_id: 3.into(), // Gem 3
            child_id: GEM_ID.into(),
            asset_id: 2,       // Asset for the kanaria which is composable
            slot_part_id: 11,  // mid gem slot
            child_asset_id: 8, // Asset id for child meant for the mid gem
        },
    );
    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ChildAssetEquipped);
    assert!(result.contains(&(ADMIN, reply.encode())));
}

pub fn compose(system: &System, token_id: TokenId, asset_id: u64) {
    let kanaria = system.get_program(KANARIA_ID);
    let (metadata, equippable_group_id, catalog_address, part_ids): (
        String,
        u64,
        ActorId,
        Vec<PartId>,
    ) = kanaria
        .read_state_using_wasm(
            "get_assets_and_equippable_data",
            WASM_BINARY.into(),
            Some((token_id, asset_id)),
        )
        .expect("Failed to read state");

    println!("Metadata {:?}", metadata);
    println!("equippable_group_id {:?}", equippable_group_id);
    println!("catalog_address {:?}", catalog_address);

    let catalog = system.get_program(CATALOG_ID);
    let catalog_state: CatalogState = catalog.read_state().expect("Failed to decode CatalogState");
    let parts = catalog_state.parts;
    let mut fixed_parts = Vec::new();
    let mut slot_parts = Vec::new();

    for part_id in part_ids.iter() {
        let (_, part) = parts.iter().find(|(id, _)| id == part_id).unwrap();
        match part {
            Part::Fixed(part) => {
                fixed_parts.push(part);
            }
            Part::Slot(part) => {
                slot_parts.push(part);
            }
        }
    }
    println!("fixed parts {:?}", fixed_parts);
    println!("slot parts {:?}", slot_parts);
}

fn add_equippable_asset_entry(
    program: &Program,
    equippable_group_id: u64,
    catalog_address: Option<ActorId>,
    metadata_uri: String,
    part_ids: Vec<PartId>,
    _id: u64,
) {
    let result = program.send(
        ADMIN,
        RMRKAction::AddEquippableAssetEntry {
            equippable_group_id,
            catalog_address,
            metadata_uri,
            part_ids,
        },
    );

    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::EquippableAssetEntryAdded);
    assert!(result.contains(&(ADMIN, reply.encode())));
}

fn set_valid_parent_for_equippable_group(
    program: &Program,
    equippable_group_id: u64,
    slot_part_id: PartId,
    parent_id: ActorId,
) {
    let result = program.send(
        ADMIN,
        RMRKAction::SetValidParentForEquippableGroup {
            equippable_group_id,
            slot_part_id,
            parent_id,
        },
    );

    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::ValidParentEquippableGroupIdSet);
    assert!(result.contains(&(ADMIN, reply.encode())));
}

fn add_asset_to_token(
    program: &Program,
    token_id: TokenId,
    asset_id: u64,
    replaces_asset_with_id: u64,
) {
    let result = program.send(
        ADMIN,
        RMRKAction::AddAssetToToken {
            token_id,
            asset_id,
            replaces_asset_with_id,
        },
    );

    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::AssetAddedToToken);
    assert!(result.contains(&(ADMIN, reply.encode())));
}

fn accept_asset(program: &Program, token_id: TokenId, asset_id: u64) {
    let res = program.send(ADMIN, RMRKAction::AcceptAsset { token_id, asset_id });
    let reply: Result<RMRKReply, RMRKError> = Ok(RMRKReply::AssetAccepted);
    assert!(res.contains(&(ADMIN, reply.encode())));
}
