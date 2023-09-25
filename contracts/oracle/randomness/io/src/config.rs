use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub manager: ActorId,
}
