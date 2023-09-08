use gmeta::{InOut, Metadata, Out};

use crate::auction::{Action, AuctionInfo, Error, Event};

pub struct AuctionMetadata;

impl Metadata for AuctionMetadata {
    type Init = ();
    type Handle = InOut<Action, Result<Event, Error>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<AuctionInfo>;
}
