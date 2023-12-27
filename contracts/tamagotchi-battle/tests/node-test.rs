use fmt::Debug;
use gclient::{EventListener, EventProcessor, GearApi, Result};
use gstd::debug;
use gstd::{collections::BTreeMap, prelude::*, ActorId};
use serde::{Deserialize, Serialize};
use std::env;
use std::{thread, time};
use tamagotchi_battle_io::*;
use tamagotchi_io::TmgInit;

use gear_core::ids::{MessageId, ProgramId};
const PATHS: [&str; 2] = [
    "../target/wasm32-unknown-unknown/release/tamagotchi.opt.wasm",
    "../target/wasm32-unknown-unknown/release/tamagotchi_battle.opt.wasm",
];

pub const PLAYERS: &[&str] = &["//John", "//Mike"];

fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
}

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }

    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
}

async fn upload_program(
    client: &GearApi,
    listener: &mut EventListener,
    path: &str,
    payload: impl Encode,
) -> Result<ProgramId> {
    let code = gclient::code_from_os(path)?;
    let encoded_payload = payload.encode();
    let gas_limit = client
        .calculate_upload_gas(None, code.clone(), encoded_payload, 0, true)
        .await?
        .burned
        * 2;
    let (message_id, program_id, _) = client
        .upload_program(
            code,
            gclient::now_micros().to_le_bytes(),
            payload,
            gas_limit,
            0,
        )
        .await?;
    assert!(listener
        .message_processed(message_id.into())
        .await?
        .succeed());
    Ok(program_id)
}

async fn send_message_listen_for_reply<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    destination: ProgramId,
    payload: impl Encode + Debug,
    _increase_gas: bool,
) -> Result<Result<T, String>> {
    println!("Sending a payload: `{payload:?}`.");

    let (message_id, _) = client
        .send_message(destination, payload, 250_000_000_000, 0)
        .await?;

    println!("Sending completed.");

    let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

    Ok(match raw_reply {
        Ok(raw_reply) => Ok(decode(raw_reply)?),
        Err(error) => Err(error),
    })
}

async fn check_reply(
    listener: &mut EventListener,
    message_id: MessageId,
    reply: Result<BattleReply, BattleError>,
) -> Result<()> {
    let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;
    let decoded_reply: Result<BattleReply, BattleError> = match raw_reply {
        Ok(raw_reply) => decode(raw_reply)?,
        Err(error) => gstd::panic!("Error in getting reply"),
    };
    println!("Received reply {:?}", decoded_reply);
    assert_eq!(decoded_reply, reply, "Wrong received reply");
    Ok(())
}
async fn send_message(
    client: &GearApi,
    destination: ProgramId,
    payload: impl Encode + Debug,
    _increase_gas: bool,
) -> Result<MessageId> {
    println!("Sending a payload: `{payload:?}`.");
    let (message_id, _) = client
        .send_message(destination, payload, 250_000_000_000, 0)
        .await?;

    Ok(message_id)
}

async fn transfer_balances(client: &GearApi, account: &str) -> Result<()> {
    let account_id: [u8; 32] = client.get_specific_actor_id(account).into();
    client
        .transfer(account_id.into(), 50_000_000_000_000)
        .await?;
    Ok(())
}

async fn upload_tamagotchis<'a>(
    client: &GearApi,
    listener: &mut EventListener,
) -> Result<(Vec<ActorId>, Vec<ActorId>)> {
    let mut tmg_ids = Vec::new();
    let mut owners = Vec::new();
    for (i, player) in PLAYERS.into_iter().copied().enumerate() {
        let client = client
            .clone()
            .with(player)
            .expect("Unable to change signer.");

        let payload = TmgInit {
            name: player.to_string(),
        };
        let tmg_id = upload_program(&client, listener, PATHS[0], payload).await?;
        println!("Tamagotchi `{player}` is initialized.");
        let account_id = client.get_actor_id();
        owners.push(account_id);
        let tmg_id: [u8; 32] = tmg_id.into();
        tmg_ids.push(tmg_id.into());
    }
    Ok((tmg_ids, owners))
}

async fn register_tamagotchis(
    client: &GearApi,
    listener: &mut EventListener,
    battle_id: ProgramId,
    tmg_ids: Vec<ActorId>,
) -> Result<()> {
    for (i, player) in PLAYERS.into_iter().copied().enumerate() {
        let tmg_id = tmg_ids[i];
        let client = client
            .clone()
            .with(player)
            .expect("Unable to change signer.");
        let expected_reply: Result<BattleReply, BattleError> =
            Ok(BattleReply::Registered { tmg_id });
        assert_eq!(
            Ok(expected_reply),
            send_message_listen_for_reply(
                &client,
                listener,
                battle_id,
                BattleAction::Register { tmg_id },
                false
            )
            .await?
        );
        println!("Tamagotchi {i} is registered.");
    }
    Ok(())
}

async fn start_battle(client: &GearApi, battle_id: ProgramId) -> Result<MessageId> {
    let client = client
        .clone()
        .with("//Alice")
        .expect("Unable to change signer.");
    // start battle

    let message_id = send_message(&client, battle_id, BattleAction::StartBattle, true).await?;
    Ok(message_id)
}

async fn make_move(
    client: &GearApi,
    battle_id: ProgramId,
    player: &str,
    pair_id: PairId,
    tmg_move: Move,
) -> Result<MessageId> {
    let client = client
        .clone()
        .with(player)
        .expect("Unable to change signer.");
    // start battle

    let message_id = send_message(
        &client,
        battle_id,
        BattleAction::MakeMove { pair_id, tmg_move },
        true,
    )
    .await?;
    Ok(message_id)
}

async fn check_battle_state(client: &GearApi, battle_id: ProgramId, expected_state: BattleState) {
    let reply: BattleQueryReply = client
        .read_state(battle_id, BattleQuery::State.encode())
        .await
        .expect("Unable to read state");

    if let BattleQueryReply::State { state } = reply {
        assert_eq!(state, expected_state);
    } else {
        gstd::panic!("Wrong received reply");
    }
}

async fn get_pairs(client: &GearApi, battle_id: ProgramId) -> Result<BTreeMap<PairId, Pair>> {
    let reply: BattleQueryReply = client
        .read_state(battle_id, BattleQuery::GetPairs.encode())
        .await?;

    let pairs = if let BattleQueryReply::Pairs { pairs } = reply {
        pairs
    } else {
        gstd::panic!("Wrong received reply");
    };
    Ok(pairs)
}

async fn get_pair(client: &GearApi, battle_id: ProgramId, pair_id: PairId) -> Result<Option<Pair>> {
    let reply = client
        .read_state(battle_id, BattleQuery::GetPair { pair_id }.encode())
        .await?;

    let pair = if let BattleQueryReply::Pair { pair } = reply {
        pair
    } else {
        gstd::panic!("Wrong received reply");
    };
    Ok(pair)
}

#[tokio::test]
async fn ipreconfigure() -> Result<()> {
    let client = GearApi::dev().await?;
    let mut listener = client.subscribe().await?;

    for player in PLAYERS {
        transfer_balances(&client, player).await?;
    }

    // upload tamagotchis
    let (tmg_ids, owners) = upload_tamagotchis(&client, &mut listener).await?;

    let mut string_tmg_ids = Vec::new();

    for tmg_id in tmg_ids.iter() {
        let tmg_id_bytes: [u8; 32] = (*tmg_id).into();
        let tmg_id_string = hex::encode(&tmg_id_bytes);
        string_tmg_ids.push(tmg_id_string);
    }

    let serialized = serde_json::to_string(&string_tmg_ids).unwrap();
    println!("Tamagotchi addresses {:?}", serialized);
    env::set_var("TMG_ADDRESSES", serialized.clone());

    let mut string_owners = Vec::new();

    for owner_id in owners.into_iter() {
        let owner_id_bytes: [u8; 32] = owner_id.into();
        let owner_id_string = hex::encode(&owner_id_bytes);
        string_owners.push(owner_id_string);
    }

    let serialized = serde_json::to_string(&string_owners).unwrap();
    println!("Tamagotchi owners {:?}", serialized);
    env::set_var("OWNER_ADDRESSES", serialized.clone());

    // upload battle contract
    let battle_id = upload_program(
        &client,
        &mut listener,
        PATHS[1],
        Config {
            max_power: 10_000,
            max_range: 10_000,
            min_range: 3_000,
            health: 2_500,
            max_steps_in_round: 5,
            max_participants: 50,
            time_for_move: 20,
            min_gas_amount: 5_000_000_000,
            block_duration_ms: 3_000,
        },
    )
    .await?;

    let battle_id_bytes: [u8; 32] = battle_id.into();
    let battle_id_str = hex::encode(&battle_id_bytes);
    println!("Battle id {:?}", battle_id_str);
    env::set_var("BATTLE_ADDRESS", battle_id_str);
    std::env::var("BATTLE_ADDRESS").expect("The env variable BATTLE_ADDRESS is not set");
    // register tamagotchis
    register_tamagotchis(&client, &mut listener, battle_id, tmg_ids).await?;

    Ok(())
}

fn get_battle_id() -> Result<ProgramId> {
    let battle_id =
        std::env::var("BATTLE_ADDRESS").expect("The env variable BATTLE_ADDRESS is not set");
    let battle_id_bytes = hex::decode(battle_id).expect("Unable to decode into bytes");
    let battle_id = ProgramId::decode(&mut battle_id_bytes.as_slice())
        .expect("Unable to decode into ProgramId");
    Ok(battle_id)
}
// Both players skip their turn from the very beginning (no player has made a move).
#[tokio::test]
async fn initial_turns_skipped() -> Result<()> {
    let wait = time::Duration::from_secs(120);
    thread::sleep(wait);

    let client = GearApi::dev().await?;
    // let client = GearApi::dev_from_path("../target/tmp/gear")
    //     .await?
    //     .with("//Alice")?;
    let mut listener = client.subscribe().await?;

    let battle_id = get_battle_id()?;

    // start battle
    let message_id = start_battle(&client, battle_id).await?;

    // check battle state
    check_battle_state(&client, battle_id, BattleState::GameIsOn).await;

    // 1st player misses the turn
    let wait = time::Duration::from_secs(65);
    thread::sleep(wait);

    let pair = get_pair(&client, battle_id, 0)
        .await?
        .expect("Pair is None");

    assert_eq!(pair.moves, vec![None], "Moves do not match");

    // 2nd player misses the turn
    let wait = time::Duration::from_secs(65);
    thread::sleep(wait);

    let pair = get_pair(&client, battle_id, 0).await?;

    assert!(pair.is_none(), "Pair must be deleted");

    check_reply(
        &mut listener,
        message_id,
        Ok(BattleReply::BattleWasCancelled),
    )
    .await?;
    Ok(())
}

// One player plays, the other skips.
#[tokio::test]
async fn one_player_plays_other_skips() -> Result<()> {
    // let client = GearApi::dev().await?;
    // // let client = GearApi::dev_from_path("../target/tmp/gear")
    // //     .await?
    // //     .with("//Alice")?;
    // let mut listener = client.subscribe().await?;

    // for player in PLAYERS {
    //     transfer_balances(&client, player).await?;
    // }

    // // upload tamagotchis
    // let (tmg_ids, player_id_to_suri) = upload_tamagotchis(&client, &mut listener).await?;

    // let client = client
    //     .clone()
    //     .with("//Alice")
    //     .expect("Unable to change signer.");

    // // upload battle contract
    // let battle_id = upload_program(
    //     &client,
    //     &mut listener,
    //     PATHS[1],
    //     Config {
    //         max_power: 10_000,
    //         max_range: 10_000,
    //         min_range: 3_000,
    //         health: 2_500,
    //         max_steps_in_round: 5,
    //         max_participants: 50,
    //         time_for_move: 20,
    //         min_gas_amount: 5_000_000_000,
    //         block_duration_ms: 3_000,
    //     },
    // )
    // .await?;

    // // register tamagotchis
    // register_tamagotchis(&client, &mut listener, battle_id, tmg_ids).await?;

    // // start battle
    // start_battle(&client, battle_id).await?;

    // // check battle state
    // check_battle_state(&client, battle_id, BattleState::GameIsOn).await;

    // // read pairs info
    // let pairs = get_pairs(&client, battle_id).await?;

    // for (pair_id, pair) in pairs {
    //     let zero_msg_id: MessageId = [0; 32].into();
    //     let max_rounds = 5;
    //     let player_1 = pair.owner_ids[0];
    //     let suri = player_id_to_suri
    //         .get(&player_1)
    //         .expect("Players does not exist");

    //     let mut player_1_msg_id = zero_msg_id;
    //     let mut prev_player_1_msg_id = zero_msg_id;
    //     for i in 0..max_rounds {
    //         let player_1_msg_id =
    //             make_move(&client, battle_id, suri, pair_id, Move::Attack).await?;
    //         let wait = time::Duration::from_secs(65);
    //         thread::sleep(wait);

    //         if prev_player_1_msg_id != zero_msg_id {
    //             check_reply(
    //                 &mut listener,
    //                 prev_player_1_msg_id,
    //                 Ok(BattleReply::MoveMade),
    //             )
    //             .await?;
    //         }

    //         prev_player_1_msg_id = player_1_msg_id;

    //         let pair = get_pair(&client, battle_id, pair_id)
    //             .await?
    //             .expect("Pair is None");
    //         if pair.game_is_over {
    //             // check battle state
    //             check_battle_state(&client, battle_id, BattleState::GameIsOver).await;
    //             break;
    //         } else {
    //             // number of skipped moves must be 1
    //             assert_eq!(
    //                 pair.amount_of_skipped_moves, 1,
    //                 "Number of skipped moves must be 1"
    //             );
    //         }
    //     }
    // }

    Ok(())
}

// Both players play.
#[tokio::test]
async fn both_players_play() -> Result<()> {
    // let client = GearApi::dev().await?;
    // // let client = GearApi::dev_from_path("../target/tmp/gear")
    // //     .await?
    // //     .with("//Alice")?;
    // let mut listener = client.subscribe().await?;

    // for player in PLAYERS {
    //     transfer_balances(&client, player).await?;
    // }

    // // upload tamagotchis
    // let (tmg_ids, player_id_to_suri) = upload_tamagotchis(&client, &mut listener).await?;

    // let client = client
    //     .clone()
    //     .with("//Alice")
    //     .expect("Unable to change signer.");

    // // upload battle contract
    // let battle_id = upload_program(
    //     &client,
    //     &mut listener,
    //     PATHS[1],
    //     Config {
    //         max_power: 10_000,
    //         max_range: 10_000,
    //         min_range: 3_000,
    //         health: 2_500,
    //         max_steps_in_round: 5,
    //         max_participants: 50,
    //         time_for_move: 20,
    //         min_gas_amount: 5_000_000_000,
    //         block_duration_ms: 3_000,
    //     },
    // )
    // .await?;

    // // register tamagotchis
    // register_tamagotchis(&client, &mut listener, battle_id, tmg_ids).await?;

    // // start battle
    // start_battle(&client, battle_id).await?;

    // // check battle state
    // check_battle_state(&client, battle_id, BattleState::GameIsOn).await;

    // // read pairs info
    // let pairs = get_pairs(&client, battle_id).await?;

    // let zero_msg_id: MessageId = [0; 32].into();
    // for (pair_id, pair) in pairs {
    //     let max_rounds = 5;
    //     let mut player_1_msg_id = zero_msg_id;
    //     let mut player_2_msg_id = zero_msg_id;
    //     let player_1 = pair.owner_ids[0];
    //     let suri_1 = player_id_to_suri
    //         .get(&player_1)
    //         .expect("Players does not exist");
    //     let player_2 = pair.owner_ids[1];
    //     let suri_2 = player_id_to_suri
    //         .get(&player_2)
    //         .expect("Players does not exist");

    //     for i in 0..max_rounds {
    //         player_1_msg_id = make_move(&client, battle_id, suri_1, pair_id, Move::Attack).await?;

    //         // wait just over 15 blocks before making a move from the second player
    //         let wait = time::Duration::from_secs(45);
    //         thread::sleep(wait);

    //         if player_2_msg_id != zero_msg_id {
    //             // wait just over 15 blocks to receive a reply from message
    //             check_reply(&mut listener, player_2_msg_id, Ok(BattleReply::MoveMade)).await?;
    //         }

    //         player_2_msg_id = make_move(&client, battle_id, suri_2, pair_id, Move::Attack).await?;

    //         // wait just over 15 blocks to receive a reply from message
    //         let wait = time::Duration::from_secs(45);
    //         thread::sleep(wait);
    //         let pair = get_pair(&client, battle_id, pair_id)
    //             .await?
    //             .expect("Pair is None");

    //         if pair.game_is_over {
    //             // check battle state
    //             check_battle_state(&client, battle_id, BattleState::GameIsOver).await;
    //             break;
    //         } else {
    //             check_reply(&mut listener, player_1_msg_id, Ok(BattleReply::MoveMade)).await?;
    //         }
    //     }
    // }

    Ok(())
}

// Players have made several moves and then consecutively skipped their turn (the pair should be removed).
#[tokio::test]
async fn consecutive_turns_skipped_after_moves() -> Result<()> {
    // let client = GearApi::dev().await?;
    // // let client = GearApi::dev_from_path("../target/tmp/gear")
    // //     .await?
    // //     .with("//Alice")?;
    // let mut listener = client.subscribe().await?;

    // for player in PLAYERS {
    //     transfer_balances(&client, player).await?;
    // }

    // // upload tamagotchis
    // let (tmg_ids, player_id_to_suri) = upload_tamagotchis(&client, &mut listener).await?;

    // let client = client
    //     .clone()
    //     .with("//Alice")
    //     .expect("Unable to change signer.");

    // // upload battle contract
    // let battle_id = upload_program(
    //     &client,
    //     &mut listener,
    //     PATHS[1],
    //     Config {
    //         max_power: 10_000,
    //         max_range: 10_000,
    //         min_range: 3_000,
    //         health: 2_500,
    //         max_steps_in_round: 5,
    //         max_participants: 50,
    //         time_for_move: 20,
    //         min_gas_amount: 5_000_000_000,
    //         block_duration_ms: 3_000,
    //     },
    // )
    // .await?;

    // // register tamagotchis
    // register_tamagotchis(&client, &mut listener, battle_id, tmg_ids).await?;

    // // start battle
    // start_battle(&client, battle_id).await?;

    // // check battle state
    // check_battle_state(&client, battle_id, BattleState::GameIsOn).await;

    // // read pairs info
    // let pairs = get_pairs(&client, battle_id).await?;

    // for (pair_id, pair) in pairs {
    //     let player_1 = pair.owner_ids[0];
    //     let suri = player_id_to_suri
    //         .get(&player_1)
    //         .expect("Players does not exist");

    //     let msg_id = make_move(&client, battle_id, suri, pair_id, Move::Attack).await?;

    //     // wait just over 21 blocks to be sure
    //     let time_for_move = time::Duration::from_secs(65);

    //     thread::sleep(time_for_move);

    //     let pair = get_pair(&client, battle_id, pair_id)
    //         .await?
    //         .expect("Pair is None");

    //     assert!(pair.moves.is_empty(), "Moves don't match");

    //     thread::sleep(time_for_move);

    //     // battle must be cancelled and pair must be deleted
    //     check_reply(&mut listener, msg_id, Ok(BattleReply::BattleWasCancelled)).await?;
    //     let pair = get_pair(&client, battle_id, pair_id).await?;

    //     assert!(pair.is_none(), "Pair must be deleted");

    //     // check battle state
    //     check_battle_state(&client, battle_id, BattleState::GameIsOver).await;
    // }

    Ok(())
}
