use super::*;

// 91 tiles genereted for one game
const TOTAL_TILES_AMOUNT: usize = 91;

#[test]
fn test_tiles_per_person() {
    assert_eq!(tiles_per_person(2), 8);
    assert_eq!(tiles_per_person(3), 8);
    assert_eq!(tiles_per_person(4), 8);
    assert_eq!(tiles_per_person(5), 7);
    assert_eq!(tiles_per_person(6), 6);
    assert_eq!(tiles_per_person(7), 5);
    assert_eq!(tiles_per_person(8), 4);
}

#[test]
fn test_is_double_tile_bigger() {
    let tiles = vec![
        Tile::new(Face::One, Face::Two),
        Tile::new(Face::Three, Face::Three),
        Tile::new(Face::Five, Face::Five),
        Tile::new(Face::Ten, Face::Eleven),
    ];

    assert_eq!(is_double_tile_bigger(1, Some((2, 0)), &tiles), false);

    assert_eq!(is_double_tile_bigger(3, Some((2, 0)), &tiles), false);

    assert_eq!(is_double_tile_bigger(1, None, &tiles), true);
}

#[test]
fn test_swap_tile() {
    let tile = Tile::new(Face::One, Face::Two);
    let swapped_tile = tile.swap();
    assert_eq!(swapped_tile.left, Face::Two);
    assert_eq!(swapped_tile.right, Face::One);
}

#[test]
fn test_is_double() {
    let tile = Tile::new(Face::One, Face::Two);
    assert_eq!(tile.is_double(), false);

    let double_tile = Tile::new(Face::Two, Face::Two);
    assert_eq!(double_tile.is_double(), true);
}

#[test]
fn test_can_adjoin() {
    let tile = Tile::new(Face::One, Face::Two);
    let other_tile = Tile::new(Face::Two, Face::Three);
    assert_eq!(tile.can_adjoin(&other_tile), true);

    let non_adjoining_tile = Tile::new(Face::Three, Face::Four);
    assert_eq!(tile.can_adjoin(&non_adjoining_tile), false);
}

#[test]
fn test_build_tile_collection() {
    let tiles = build_tile_collection();
    assert_eq!(tiles.len(), TOTAL_TILES_AMOUNT);

    let mut set = BTreeSet::new();
    for tile in tiles {
        assert!(set.insert(tile));
    }
}

#[test]
fn test_give_tiles_until_double() {
    let players_amount = 3;
    let mut remaining_tiles: BTreeSet<u32> = Default::default();

    let tiles = vec![
        Tile::new(Face::Zero, Face::One),
        Tile::new(Face::Two, Face::Two),
        Tile::new(Face::Three, Face::Zero),
    ];

    for i in 0..tiles.len() {
        remaining_tiles.insert(i as u32);
    }

    let mut tile_to_player = Default::default();

    let matching_tile_id = give_tiles_until_double(
        &mut remaining_tiles,
        &tiles,
        &mut tile_to_player,
        players_amount,
    );

    // Verify that everyone has one tile
    assert_eq!(tile_to_player.len(), players_amount);

    assert!(matching_tile_id.is_some());

    let start_tile = tiles[matching_tile_id.unwrap().0 as usize];
    assert!(start_tile.is_double());
    assert_eq!(start_tile.left, Face::Two);
    assert_eq!(matching_tile_id.unwrap().1, 1);
}

#[test]
fn test_give_tiles_until_double_2() {
    let players_amount = 5;
    let mut remaining_tiles: BTreeSet<u32> = Default::default();

    let tiles = vec![
        Tile::new(Face::Zero, Face::One),
        Tile::new(Face::Two, Face::One),
        Tile::new(Face::Three, Face::Zero),
        Tile::new(Face::Zero, Face::One),
        Tile::new(Face::Two, Face::Eleven),
    ];

    for i in 0..tiles.len() {
        remaining_tiles.insert(i as u32);
    }

    let mut tile_to_player = Default::default();

    let matching_tile_id = give_tiles_until_double(
        &mut remaining_tiles,
        &tiles,
        &mut tile_to_player,
        players_amount,
    );

    // Verify that everyone has one tile
    assert_eq!(tile_to_player.len(), players_amount);

    assert!(matching_tile_id.is_none());
}

#[test]
fn test_give_tiles_until_double_3() {
    let players_amount = 5;
    let mut remaining_tiles: BTreeSet<u32> = Default::default();

    let tiles = vec![
        Tile::new(Face::Zero, Face::One),
        Tile::new(Face::Two, Face::Two),
        Tile::new(Face::Three, Face::Zero),
        Tile::new(Face::Zero, Face::One),
        Tile::new(Face::Eleven, Face::Eleven),
        Tile::new(Face::Twelve, Face::Eleven),
        Tile::new(Face::Eleven, Face::Twelve),
    ];

    for i in 0..tiles.len() {
        remaining_tiles.insert(i as u32);
    }

    let mut tile_to_player = Default::default();

    let matching_tile_id = give_tiles_until_double(
        &mut remaining_tiles,
        &tiles,
        &mut tile_to_player,
        players_amount,
    );

    // Verify that everyone has one tile
    assert_eq!(tile_to_player.len(), players_amount);

    assert!(matching_tile_id.is_some());

    let start_tile = tiles[matching_tile_id.unwrap().0 as usize];
    assert!(start_tile.is_double());
    assert_eq!(start_tile.left, Face::Eleven);
    assert_eq!(matching_tile_id.unwrap().1, 4);
}

#[test]
fn test_game_state_fail_init() {
    let actor1 = ActorId::new([1u8; 32]);
    let players = Players {
        players: vec![actor1],
    };
    let game_state = GameState::new(&players);
    assert!(game_state.is_none());

    let players = Players {
        players: vec![actor1; 9],
    };
    let game_state = GameState::new(&players);
    assert!(game_state.is_none());
}

#[test]
fn test_game_state() {
    let actor1 = ActorId::new([1u8; 32]);
    let actor2 = ActorId::new([2u8; 32]);
    let players = Players {
        players: vec![actor1, actor2],
    };

    let game_state = GameState::new(&players).unwrap();

    let mut counters = (0u32, 0u32);
    for player_id in game_state.tile_to_player.clone().values() {
        match *player_id {
            0 => counters.0 += 1,
            1 => counters.1 += 1,
            _ => unreachable!("test failed cause of invalid player ID"),
        }
    }

    assert_eq!(counters.0, 8); // 8 tiles
    assert_eq!(counters.1, 7); // 7 tiles, 1 is starting

    assert_eq!(
        game_state.tiles[game_state.start_tile as usize],
        Tile {
            right: Face::One,
            left: Face::One,
        }
    );
    assert_eq!(game_state.current_player, 0); // First Actor starts
    assert_eq!(game_state.tiles.len(), TOTAL_TILES_AMOUNT);

    // Check that all tiles are given and not repeated
    let mut tiles_indicator = [false; TOTAL_TILES_AMOUNT];

    tiles_indicator[game_state.start_tile as usize] = true;
    for id in game_state.remaining_tiles {
        assert_eq!(tiles_indicator[id as usize], false);
        tiles_indicator[id as usize] = true;
    }

    for id in game_state.tile_to_player.keys() {
        assert_eq!(tiles_indicator[*id as usize], false);
        tiles_indicator[*id as usize] = true;
    }

    for indicator in tiles_indicator {
        assert!(indicator);
    }
}
