use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use supply_chain_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn item_info(item_id: ItemId, state: Self::State) -> Option<ItemInfo> {
        state.item_info(item_id)
    }

    fn participants(state: Self::State) -> Participants {
        state.participants()
    }

    fn roles(actor: ActorId, state: Self::State) -> Vec<Role> {
        state.roles(actor)
    }

    fn existing_items(state: Self::State) -> Vec<(ItemId, ItemInfo)> {
        state.items
    }

    fn fungible_token(state: Self::State) -> ActorId {
        state.fungible_token
    }

    fn non_fungible_token(state: Self::State) -> ActorId {
        state.non_fungible_token
    }

    fn is_action_cached(actor_action: ActorIdInnerSupplyChainAction, state: Self::State) -> bool {
        let (actor, action) = actor_action;

        state.is_action_cached(actor, action)
    }
}

// #[metawasm] doesn't process the explicit tuple type ¯\_(ツ)_/¯.
pub type ActorIdInnerSupplyChainAction = (ActorId, InnerAction);
