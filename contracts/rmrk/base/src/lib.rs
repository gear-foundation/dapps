#![no_std]

use base_io::*;
use gstd::{msg, prelude::*, ActorId};
use types::primitives::*;

#[derive(Debug, Default)]
pub struct Base {
    pub issuer: ActorId,
    pub base_type: String,
    pub symbol: String,
    pub parts: BTreeMap<PartId, Part>,
}

static mut BASE: Option<Base> = None;

impl Base {
    fn add_parts(&mut self, parts: BTreeMap<PartId, Part>) {
        assert!(msg::source() == self.issuer, "wrong issuer");
        for (part_id, part) in parts.clone() {
            assert!(
                self.parts.insert(part_id, part).is_none(),
                "The part with that the one of given `PartId`'s already exists"
            );
        }
        msg::reply(BaseEvent::PartsAdded(parts), 0)
            .expect("Error in reply `[BaseEvent::PartsAdded]`");
    }

    fn remove_parts(&mut self, parts: Vec<PartId>) {
        assert!(msg::source() == self.issuer, "wrong issuer");
        for part_id in parts.clone() {
            assert!(
                self.parts.remove(&part_id).is_some(),
                "One of given parts does not exist"
            );
        }
        msg::reply(BaseEvent::PartsRemoved(parts), 0)
            .expect("Error in reply `[BaseEvent::PartsRemoved]`");
    }

    fn add_equippable(&mut self, part_id: PartId, collection_id: CollectionId, token_id: TokenId) {
        assert!(msg::source() == self.issuer, "wrong issuer");
        let part = self
            .parts
            .get_mut(&part_id)
            .expect("Part with that id does not exist");
        if let Part::Slot(SlotPart { equippable, .. }) = part {
            if let EquippableList::Custom(collection_and_token) = equippable {
                collection_and_token.insert((collection_id, token_id));
            }
        } else {
            panic!("Equippable can be added only to SlotPart");
        }
        msg::reply(
            BaseEvent::EquippableAdded {
                part_id,
                collection_id,
                token_id,
            },
            0,
        )
        .expect("Error in reply `[BaseEvent::EquippableAdded]`");
    }

    fn remove_equippable(
        &mut self,
        part_id: PartId,
        collection_id: CollectionId,
        token_id: TokenId,
    ) {
        assert!(msg::source() == self.issuer, "wrong issuer");
        let part = self
            .parts
            .get_mut(&part_id)
            .expect("Part with that id does not exist");
        if let Part::Slot(SlotPart { equippable, .. }) = part {
            if let EquippableList::Custom(collection_and_token) = equippable {
                collection_and_token.remove(&(collection_id, token_id));
            }
        } else {
            panic!("Equippable can be removed only from SlotPart");
        }
        msg::reply(
            BaseEvent::EquippableRemoved {
                part_id,
                collection_id,
                token_id,
            },
            0,
        )
        .expect("Error in reply `[BaseEvent::EquippableAdded]`");
    }

    fn check_part(&self, part_id: PartId) {
        let part = self
            .parts
            .get(&part_id)
            .expect("Part with that id does not exist");
        msg::reply(BaseEvent::Part(part.clone()), 0).expect("Error in reply `[BaseEvent::Part]`");
    }

    fn check_equippable(&self, part_id: PartId, collection_id: CollectionId, token_id: TokenId) {
        let part = self
            .parts
            .get(&part_id)
            .expect("Part with that id does not exist");
        if let Part::Slot(SlotPart { equippable, .. }) = part {
            if let EquippableList::Custom(collection_and_token) = equippable {
                if !collection_and_token.contains(&(collection_id, token_id)) {
                    panic!("Token is not in equippable list")
                }
            }
        } else {
            panic!("The part must be slot");
        }

        msg::reply(BaseEvent::InEquippableList, 0)
            .expect("Error in reply `[BaseEvent::InEquippableList]`");
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitBase = msg::load().expect("Unable to decode InitBase");
    let base = Base {
        issuer: msg::source(),
        base_type: config.base_type,
        symbol: config.symbol,
        ..Default::default()
    };
    unsafe {
        BASE = Some(base);
    }
}
#[no_mangle]
extern "C" fn handle() {
    let action: BaseAction = msg::load().expect("Could not load BaseAction");
    let base = unsafe { BASE.get_or_insert(Default::default()) };
    match action {
        BaseAction::AddParts(parts) => base.add_parts(parts),
        BaseAction::AddEquippable {
            part_id,
            collection_id,
            token_id,
        } => base.add_equippable(part_id, collection_id, token_id),
        BaseAction::RemoveParts(parts) => base.remove_parts(parts),
        BaseAction::RemoveEquippable {
            part_id,
            collection_id,
            token_id,
        } => base.remove_equippable(part_id, collection_id, token_id),
        BaseAction::CheckPart(part_id) => base.check_part(part_id),
        BaseAction::CheckEquippable {
            part_id,
            collection_id,
            token_id,
        } => base.check_equippable(part_id, collection_id, token_id),
    }
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: BaseState = msg::load().expect("failed to decode BaseState");
    let base = BASE.get_or_insert(Default::default());

    let encoded = match query {
        BaseState::Parts => {
            let parts: Vec<Part> = base.parts.values().cloned().collect();
            BaseStateReply::Parts(parts)
        }
        BaseState::Part(part_id) => {
            if let Some(part) = base.parts.get(&part_id) {
                BaseStateReply::Part(Some(part.clone()))
            } else {
                BaseStateReply::Part(None)
            }
        }
        BaseState::IsEquippable {
            part_id,
            collection_id,
            token_id,
        } => {
            if let Some(Part::Slot(SlotPart { equippable, .. })) = base.parts.get(&part_id) {
                match equippable {
                    EquippableList::Custom(collection_and_token) => {
                        if collection_and_token.contains(&(collection_id, token_id)) {
                            BaseStateReply::IsEquippable(true)
                        } else {
                            BaseStateReply::IsEquippable(false)
                        }
                    }
                    _ => BaseStateReply::IsEquippable(true),
                }
            } else {
                BaseStateReply::IsEquippable(false)
            }
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}
