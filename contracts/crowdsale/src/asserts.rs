use gstd::{msg, ActorId};

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

pub fn owner_message(owner: &ActorId, message: &str) {
    if msg::source() != *owner {
        panic!("{}: Not owner message", message)
    }
}

pub fn not_zero_address(address: &ActorId, message: &str) {
    if address == &ZERO_ID {
        panic!("{}: Zero address", message)
    }
}
