use gstd::{prelude::*, Decode, Encode, TypeInfo};

/// Used to represent high and low parts of unsigned 256-bit integer.
pub type RandomSeed = (u128, u128);

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Random {
    pub randomness: RandomSeed,
    pub signature: String,
    pub prev_signature: String,
}
