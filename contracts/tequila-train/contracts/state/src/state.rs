use gmeta::metawasm;
use gstd::prelude::*;

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;
}
