#![no_std]

use base_io::*;
use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;
use types::primitives::*;

#[derive(Debug, Default)]
pub struct Base {
    /// Original creator of the Base.
    pub issuer: ActorId,

    /// Specifies how an NFT should be rendered, ie "svg".
    pub base_type: String,

    /// Provided by user during Base creation.
    pub symbol: String,

    /// Parts that the base has.
    /// Mapping from `PartId` to fixed or slot `Part`.
    pub parts: HashMap<PartId, Part>,
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
extern "C" fn state() {
    let base = unsafe { BASE.as_ref().expect("Base is not initialized") };
    let base_state = BaseState {
        issuer: base.issuer,
        base_type: base.base_type.clone(),
        symbol: base.symbol.clone(),
        parts: base
            .parts
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
    };
    msg::reply(base_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
