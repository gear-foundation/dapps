use super::{GamePairsMap, MultipleGamesMap};

crate::declare_storage!(module: multiple_games, name: MultipleGamesStorage, ty: MultipleGamesMap);
crate::declare_storage!(module: pairs, name: GamePairsStorage, ty: GamePairsMap);
