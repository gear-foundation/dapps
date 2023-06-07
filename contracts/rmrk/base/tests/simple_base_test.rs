use base_io::*;
use gstd::{prelude::*, BTreeMap, BTreeSet};
use gtest::{Program, RunResult, System};
use types::primitives::PartId;
pub const ISSUER: u64 = 10;

pub fn init_base(sys: &System, issuer: u64) {
    sys.init_logger();
    let rmrk = Program::current(sys);
    let res = rmrk.send(
        issuer,
        InitBase {
            base_type: "svg".to_string(),
            symbol: "BaseSymbol".to_string(),
        },
    );
    assert!(res.log().is_empty());
}

fn get_parts() -> BTreeMap<PartId, Part> {
    let mut parts: BTreeMap<PartId, Part> = BTreeMap::new();
    let fixed_part_id = 100;
    let fixed_part = Part::Fixed(FixedPart {
        z: Some(3),
        src: "fixed_part_src".to_string(),
    });
    parts.insert(fixed_part_id, fixed_part);

    let slot_part_id = 102;
    let slot_part = Part::Slot(SlotPart {
        equippable: EquippableList::Custom(BTreeSet::new()),
        z: Some(3),
        src: "slot_part_src".to_string(),
    });
    parts.insert(slot_part_id, slot_part);

    let slot_part_id = 103;
    let slot_part = Part::Slot(SlotPart {
        equippable: EquippableList::All,
        z: Some(2),
        src: "slot_part_src".to_string(),
    });

    parts.insert(slot_part_id, slot_part);
    parts
}
fn add_parts(base: &Program, issuer: u64, parts: BTreeMap<PartId, Part>) {
    let res = base.send(issuer, BaseAction::AddParts(parts.clone()));
    assert!(res.contains(&(issuer, BaseEvent::PartsAdded(parts).encode())));
}

fn check_part(base: &Program, part_id: PartId) -> RunResult {
    base.send(ISSUER, BaseAction::CheckPart(part_id))
}

fn remove_parts(base: &Program, issuer: u64, parts: Vec<PartId>) -> RunResult {
    base.send(issuer, BaseAction::RemoveParts(parts))
}

fn add_equippable(
    base: &Program,
    issuer: u64,
    part_id: PartId,
    collection_id: u64,
    token_id: u64,
) -> RunResult {
    base.send(
        issuer,
        BaseAction::AddEquippable {
            part_id,
            collection_id: collection_id.into(),
            token_id: token_id.into(),
        },
    )
}

fn remove_equippable(
    base: &Program,
    issuer: u64,
    part_id: PartId,
    collection_id: u64,
    token_id: u64,
) -> RunResult {
    base.send(
        issuer,
        BaseAction::RemoveEquippable {
            part_id,
            collection_id: collection_id.into(),
            token_id: token_id.into(),
        },
    )
}

fn check_equippable(
    base: &Program,
    part_id: PartId,
    collection_id: u64,
    token_id: u64,
) -> RunResult {
    base.send(
        ISSUER,
        BaseAction::CheckEquippable {
            part_id,
            collection_id: collection_id.into(),
            token_id: token_id.into(),
        },
    )
}

#[test]
fn add_parts_test() {
    let sys = System::new();
    init_base(&sys, ISSUER);
    let base = sys.get_program(1);
    let parts = get_parts();
    add_parts(&base, ISSUER, parts);

    // // meta state (parts)
    // let parts_reply: BaseStateReply = base
    //     .meta_state(BaseState::Parts)
    //     .expect("Meta_state failed");
    // let expected_reply: Vec<Part> = parts.values().cloned().collect();
    // assert_eq!(parts_reply, BaseStateReply::Parts(expected_reply));

    // // meta state (part)
    // for (part_id, part) in parts {
    //     let part_reply: BaseStateReply = base
    //         .meta_state(BaseState::Part(part_id))
    //         .expect("Meta_state failed");
    //     assert_eq!(part_reply, BaseStateReply::Part(Some(part.clone())));
    //     // message: check parts
    //     let res = check_part(&base, part_id);
    //     assert!(res.contains(&(ISSUER, BaseEvent::Part(part).encode())));
    // }

    // // meta state for non-existing part
    // let part_reply: BaseStateReply = base
    //     .meta_state(BaseState::Part(1000))
    //     .expect("Meta_state failed");
    // assert_eq!(part_reply, BaseStateReply::Part(None));
}

#[test]
#[should_panic]
fn add_parts_wrong_issuer() {
    let sys = System::new();
    init_base(&sys, 1000);
    let base = sys.get_program(1);
    add_parts(&base, ISSUER, BTreeMap::new());
}

#[test]
fn remove_parts_test() {
    let sys = System::new();
    init_base(&sys, ISSUER);
    let base = sys.get_program(1);
    let mut parts = get_parts();
    add_parts(&base, ISSUER, parts.clone());

    let removed_parts: Vec<PartId> = vec![100, 102];
    let res = remove_parts(&base, ISSUER, removed_parts.clone());
    assert!(res.contains(&(ISSUER, BaseEvent::PartsRemoved(removed_parts).encode())));
    parts.remove(&100);
    parts.remove(&102);
    // // meta state (parts)
    // let parts_reply: BaseStateReply = base
    //     .meta_state(BaseState::Parts)
    //     .expect("Meta_state failed");
    // assert_eq!(
    //     parts_reply,
    //     BaseStateReply::Parts(vec![parts[&103].clone()])
    // );

    // // check that removed parts are None
    // let part_reply: BaseStateReply = base
    //     .meta_state(BaseState::Part(100))
    //     .expect("Meta_state failed");
    // assert_eq!(part_reply, BaseStateReply::Part(None));

    // let part_reply: BaseStateReply = base
    //     .meta_state(BaseState::Part(102))
    //     .expect("Meta_state failed");
    // assert_eq!(part_reply, BaseStateReply::Part(None));
}

#[test]
fn remove_parts_failures() {
    let sys = System::new();
    init_base(&sys, ISSUER);
    let base = sys.get_program(1);
    let parts = get_parts();
    add_parts(&base, ISSUER, parts);

    let mut removed_parts: Vec<PartId> = vec![100, 102];

    // wrong issuer
    assert!(remove_parts(&base, 1000, removed_parts.clone()).main_failed());

    removed_parts[0] = 500;
    // Part with indicated ID does not exist
    assert!(remove_parts(&base, ISSUER, removed_parts).main_failed());
}

#[test]
fn add_remove_equippable_test() {
    let sys = System::new();
    init_base(&sys, ISSUER);
    let base = sys.get_program(1);
    let mut parts = get_parts();
    add_parts(&base, ISSUER, parts.clone());

    let part_id: PartId = 102;
    let collection_id: u64 = 300;
    let token_id: u64 = 250;

    let res = add_equippable(&base, ISSUER, part_id, collection_id, token_id);
    assert!(res.contains(&(
        ISSUER,
        BaseEvent::EquippableAdded {
            part_id,
            collection_id: collection_id.into(),
            token_id: token_id.into(),
        }
        .encode()
    )));

    // check that part contains equippable
    let part = parts
        .get_mut(&part_id)
        .expect("Part with that id does not exist");
    if let Part::Slot(SlotPart {
        equippable: EquippableList::Custom(collection_and_token),
        ..
    }) = part
    {
        collection_and_token.insert((collection_id.into(), token_id.into()));
    }
    // let part_reply: BaseStateReply = base
    //     .meta_state(BaseState::Part(102))
    //     .expect("Meta_state failed");
    // assert_eq!(part_reply, BaseStateReply::Part(Some(part.clone())));

    // // check if token from the collection in the equippable list
    // let is_equippable_reply: BaseStateReply = base
    //     .meta_state(BaseState::IsEquippable {
    //         part_id,
    //         collection_id: collection_id.into(),
    //         token_id: token_id.into(),
    //     })
    //     .expect("Meta_state failed");
    // assert_eq!(is_equippable_reply, BaseStateReply::IsEquippable(true));

    // check if token from the collection in the equippable list through the message
    let res = check_equippable(&base, part_id, collection_id, token_id);
    assert!(res.contains(&(ISSUER, BaseEvent::InEquippableList.encode())));

    // // check that `is_equippable` is true if equippableList = EquippableList::All
    // let is_equippable_reply: BaseStateReply = base
    //     .meta_state(BaseState::IsEquippable {
    //         part_id: 103,
    //         collection_id: collection_id.into(),
    //         token_id: token_id.into(),
    //     })
    //     .expect("Meta_state failed");
    // assert_eq!(is_equippable_reply, BaseStateReply::IsEquippable(true));

    let res = remove_equippable(&base, ISSUER, part_id, collection_id, token_id);
    assert!(res.contains(&(
        ISSUER,
        BaseEvent::EquippableRemoved {
            part_id,
            collection_id: collection_id.into(),
            token_id: token_id.into(),
        }
        .encode()
    )));

    // // check if token from the collection is not in the equippable list
    // let is_equippable_reply: BaseStateReply = base
    //     .meta_state(BaseState::IsEquippable {
    //         part_id,
    //         collection_id: collection_id.into(),
    //         token_id: token_id.into(),
    //     })
    //     .expect("Meta_state failed");
    // assert_eq!(is_equippable_reply, BaseStateReply::IsEquippable(false));
}

#[test]
fn add_remove_equippable_failures() {
    let sys = System::new();
    init_base(&sys, ISSUER);
    let base = sys.get_program(1);
    let parts = get_parts();
    add_parts(&base, ISSUER, parts);

    let part_id: PartId = 102;
    let collection_id: u64 = 300;
    let token_id: u64 = 250;

    // must fail since part does not exist
    assert!(add_equippable(&base, ISSUER, 500, collection_id, token_id).main_failed());

    // must fail since wrong issuer
    assert!(add_equippable(&base, 500, part_id, collection_id, token_id).main_failed());

    // must fail since equippable is added to FixedPart
    assert!(add_equippable(&base, ISSUER, 100, collection_id, token_id).main_failed());

    // must fail since part does not exist
    assert!(remove_equippable(&base, ISSUER, 500, collection_id, token_id).main_failed());

    // must fail since wrong issuer
    assert!(remove_equippable(&base, 500, part_id, collection_id, token_id).main_failed());

    // must fail since equippable is removed from FixedPart
    assert!(remove_equippable(&base, ISSUER, 100, collection_id, token_id).main_failed());
}

#[test]
fn add_check_failures() {
    let sys = System::new();
    init_base(&sys, ISSUER);
    let base = sys.get_program(1);
    let parts = get_parts();
    add_parts(&base, ISSUER, parts);
    // must fail since part does not exist
    assert!(check_part(&base, 300).main_failed());

    // must fail since token is not in equippable list
    assert!(check_equippable(&base, 102, 100, 100).main_failed());
}
