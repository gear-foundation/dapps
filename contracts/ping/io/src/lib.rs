#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;

pub struct DemoPingMetadata;

impl Metadata for DemoPingMetadata {
    type Init = ();
    type Handle = InOut<String, String>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Vec<String>;
}
