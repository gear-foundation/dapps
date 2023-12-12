use fmt::Debug;
use gclient::{EventListener, EventProcessor, GearApi, Result};
use gstd::{collections::BTreeMap, prelude::*, ActorId};
use tamagotchi_battle_io::*;
use tamagotchi_io::TmgInit;

use gear_core::ids::ProgramId;
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

async fn send_message<T: Decode>(
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

async fn transfer_balances(client: &GearApi, account: &str) -> Result<()> {
    let account_id: [u8; 32] = client.get_specific_actor_id(account).into();
    client
        .transfer(account_id.into(), 50_000_000_000_000)
        .await?;
    Ok(())
}

async fn upload_tamagotchis(
    client: &GearApi,
    listener: &mut EventListener,
) -> Result<Vec<ActorId>> {
    let mut tmg_ids: Vec<ActorId> = Vec::new();
    for player in PLAYERS.into_iter() {
        let client = client
            .clone()
            .with(player)
            .expect("Unable to change signer.");

        let payload = TmgInit {
            name: player.to_string(),
        };
        let tmg_id = upload_program(&client, listener, PATHS[0], payload).await?;
        println!("Tamagotchi `{player}` is initialized.");
        let tmg_id: [u8; 32] = tmg_id.into();
        tmg_ids.push(tmg_id.into());
    }
    Ok(tmg_ids)
}

async fn register_tamagotchis(
    client: &GearApi,
    listener: &mut EventListener,
    battle_id: ProgramId,
    tmg_ids: Vec<ActorId>,
) -> Result<()> {
    for i in 0..PLAYERS.len() {
        let tmg_id = tmg_ids[i];
        let client = client
            .clone()
            .with(PLAYERS[i])
            .expect("Unable to change signer.");
        let expected_reply: Result<BattleReply, BattleError> =
            Ok(BattleReply::Registered { tmg_id });
        assert_eq!(
            Ok(expected_reply),
            send_message(
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

async fn start_battle(
    client: &GearApi,
    listener: &mut EventListener,
    battle_id: ProgramId,
) -> Result<()> {
    let client = client
        .clone()
        .with("//Alice")
        .expect("Unable to change signer.");
    // start battle
    let expected_reply: Result<BattleReply, BattleError> = Ok(BattleReply::BattleStarted);
    assert!(
        Ok(expected_reply)
            == send_message(
                &client,
                listener,
                battle_id,
                BattleAction::StartBattle,
                true
            )
            .await?
    );
    Ok(())
}
#[tokio::test]
async fn single_pair() -> Result<()> {
    // let address = WSAddress::new("wss://node-workshop.gear.rs", 443);
    let client = GearApi::dev().await?;
    // let client = GearApi::dev_from_path("../target/tmp/gear")
    //     .await?
    //     .with("//Alice")?;
    let mut listener = client.subscribe().await?;

    for player in PLAYERS {
        transfer_balances(&client, player).await?;
    }
    // let mut tmg_ids: Vec<ActorId> = Vec::new();
    // let mut actor_ids_to_str: BTreeMap<ActorId, &str> = BTreeMap::new();

    // upload tamagotchis
    let tmg_ids = upload_tamagotchis(&client, &mut listener).await?;

    let client = client
        .clone()
        .with("//Alice")
        .expect("Unable to change signer.");

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

    // let battle_id_hex = hex::encode(battle_id);
    // println!("BATTLE ID {:?}", battle_id_hex);

    // register tamagotchis
    register_tamagotchis(&client, &mut listener, battle_id, tmg_ids).await?;

    // start battle
    start_battle(&client, &mut listener, battle_id).await?;

    // // read pair_ids for players
    // for player in PLAYERS.iter() {
    //     let player_id = client.get_specific_actor_id(player);
    //     let pair_ids: Vec<PairId> = client
    //         .read_state_using_wasm_by_path(
    //             battle_id.into(),
    //             vec![],
    //             "pairs_for_player",
    //             META_WASM,
    //             Some(player_id),
    //         )
    //         .await?;
    //     println!(" Pairs {:?} for players {:?} ", pair_ids, player);
    // }

    // let mut battle_state = BattleState::GameIsOn;

    // while battle_state != BattleState::GameIsOver {
    //     let tmg_ids: Vec<ActorId> = client
    //         .read_state_using_wasm_by_path(
    //             battle_id.into(),
    //             vec![],
    //             "tmg_ids",
    //             META_WASM,
    //             <Option<()>>::None,
    //         )
    //         .await?;
    //     println!("Tmg Ids {:?}", tmg_ids);

    //     if battle_state == BattleState::WaitNextRound {
    //         let client = client
    //             .clone()
    //             .with("//Alice")
    //             .expect("Unable to change signer.");
    //         // start battle
    //         assert_eq!(
    //             Ok(BattleEvent::BattleStarted),
    //             send_message(
    //                 &client,
    //                 &mut listener,
    //                 battle_id,
    //                 BattleAction::StartBattle,
    //                 true
    //             )
    //             .await?
    //         );
    //     }
    //     let pair_ids: Vec<PairId> = client
    //         .read_state_using_wasm_by_path(
    //             battle_id.into(),
    //             vec![],
    //             "pair_ids",
    //             META_WASM,
    //             <Option<()>>::None,
    //         )
    //         .await?;
    //     println!("Current Pair Ids {:?}", pair_ids);
    //     for pair_id in pair_ids.iter() {
    //         let mut game_is_over = false;
    //         while !game_is_over {
    //             let pair: Pair = client
    //                 .read_state_using_wasm_by_path(
    //                     battle_id.into(),
    //                     vec![],
    //                     "pair",
    //                     META_WASM,
    //                     Some(pair_id),
    //                 )
    //                 .await?;

    //             let (power, health): (u16, u16) = client
    //                 .read_state_using_wasm_by_path(
    //                     battle_id.into(),
    //                     vec![],
    //                     "power_and_health",
    //                     META_WASM,
    //                     Some(pair.tmg_ids[0]),
    //                 )
    //                 .await?;

    //             println!(
    //                 "Power {:?} and health {:?} for first tamagotchi",
    //                 power, health
    //             );

    //             let (power, health): (u16, u16) = client
    //                 .read_state_using_wasm_by_path(
    //                     battle_id.into(),
    //                     vec![],
    //                     "power_and_health",
    //                     META_WASM,
    //                     Some(pair.tmg_ids[1]),
    //                 )
    //                 .await?;

    //             println!(
    //                 "Power {:?} and health {:?} for first tamagotchi",
    //                 power, health
    //             );

    //             let current_player: ActorId = client
    //                 .read_state_using_wasm_by_path(
    //                     battle_id.into(),
    //                     vec![],
    //                     "current_turn",
    //                     META_WASM,
    //                     Some(pair_id),
    //                 )
    //                 .await?;
    //             let player = actor_ids_to_str.get(&current_player).unwrap();
    //             println!("current pair {:?} current_player {:?}", pair_id, player);
    //             let client = client
    //                 .clone()
    //                 .with(player)
    //                 .expect("Unable to change signer.");
    //             assert!(
    //                 Ok(BattleEvent::MoveMade)
    //                     == send_message(
    //                         &client,
    //                         &mut listener,
    //                         battle_id,
    //                         BattleAction::MakeMove {
    //                             pair_id: *pair_id,
    //                             tmg_move: Move::Attack
    //                         },
    //                         true,
    //                     )
    //                     .await?
    //             );
    //             game_is_over = client
    //                 .read_state_using_wasm_by_path(
    //                     battle_id.into(),
    //                     vec![],
    //                     "game_is_over",
    //                     META_WASM,
    //                     Some(pair_id),
    //                 )
    //                 .await?;
    //         }
    //     }
    //     battle_state = client
    //         .read_state_using_wasm_by_path(
    //             battle_id.into(),
    //             vec![],
    //             "battle_state",
    //             META_WASM,
    //             <Option<()>>::None,
    //         )
    //         .await?;
    //     println!("Battle state {:?}", battle_state);
    // }
    // // first round
    // println!("First round");
    // let round: Round = client
    //     .read_state_using_wasm_by_path(battle_id.into(), "round", META_WASM, <Option<()>>::None)
    //     .await?;
    // println!(" {round:?} ");

    // let mut player_0 = round.tmg_ids[0];
    // let mut player_1 = round.tmg_ids[1];

    // while (true) {
    //     let (power_0, health_0) = get_power_and_health(&client, battle_id, &player_0).await;
    //     let (power_1, health_1) = get_power_and_health(&client, battle_id, &player_1).await;

    //     println!("-----");
    //     println!("tamagotchi 0 power {power_0} and health {health_0}");
    //     println!("tamagotchi 1 power {power_1} and health {health_1}");
    //     println!("-----");
    //     let move_0 = if health_0 < 1000 && power_1 > power_0 {
    //         Move::Defence
    //     } else {
    //         Move::Attack
    //     };

    //     let move_1 = if health_1 < 1000 && power_0 > power_1 {
    //         Move::Defence
    //     } else {
    //         Move::Attack
    //     };

    // assert!(
    //     Ok(BattleEvent::MoveMade)
    //         == send_message(
    //             &client,
    //             &mut listener,
    //             battle_id,
    //             BattleAction::MakeMove(move_0),
    //             true,
    //         )
    //         .await?
    // );
    // assert!(
    //     Ok(BattleEvent::MoveMade)
    //         == send_message(
    //             &client,
    //             &mut listener,
    //             battle_id,
    //             BattleAction::MakeMove(move_1),
    //             true,
    //         )
    //         .await?
    // );

    // let state: BattleState = client
    //     .read_state_using_wasm_by_path(
    //         battle_id.into(),
    //         "battle_state",
    //         META_WASM,
    //         <Option<()>>::None,
    //     )
    //     .await?;

    // if state == BattleState::WaitNextRound {
    //     println!("-----");
    //     println!("Next round");

    //     assert!(
    //         Ok(BattleEvent::NewRound)
    //             == send_message(
    //                 &client,
    //                 &mut listener,
    //                 battle_id,
    //                 BattleAction::StartNewRound,
    //                 true,
    //             )
    //             .await?
    //     );

    //         let round: Round = client
    //             .read_state_using_wasm_by_path(
    //                 battle_id.into(),
    //                 "round",
    //                 META_WASM,
    //                 <Option<()>>::None,
    //             )
    //             .await?;
    //         player_0 = round.tmg_ids[0];
    //         player_1 = round.tmg_ids[1];
    //         println!(" {round:?} ");
    //     }
    //     if state == BattleState::GameIsOver {
    //         break;
    //     }
    // }

    Ok(())
}
