use gclient::{EventListener, EventProcessor, GearApi, Result, WSAddress};

use battle_io::*;
use fmt::Debug;
use gstd::{prelude::*, ActorId};

const PATHS: [&str; 2] = [
    "../target/wasm32-unknown-unknown/release/tmg.opt.wasm",
    "../target/wasm32-unknown-unknown/release/battle.opt.wasm",
];
const META_WASM: &str = "../target/wasm32-unknown-unknown/release/battle_state.meta.wasm";

pub const PLAYERS: &[&str] = &[
    "//John", "//Mike", "//Dan", "//Bot", "//Jack", "//Mops", "//Alex",
];

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

async fn common_upload_program(
    client: &GearApi,
    code: Vec<u8>,
    payload: impl Encode,
) -> Result<([u8; 32], [u8; 32])> {
    let encoded_payload = payload.encode();
    let gas_limit = client
        .calculate_upload_gas(None, code.clone(), encoded_payload, 0, true)
        .await?
        .min_limit;
    let (message_id, program_id, _) = client
        .upload_program(
            code,
            gclient::now_in_micros().to_le_bytes(),
            payload,
            gas_limit,
            0,
        )
        .await?;

    Ok((message_id.into(), program_id.into()))
}
async fn upload_program(
    client: &GearApi,
    listener: &mut EventListener,
    path: &str,
    payload: impl Encode,
) -> Result<[u8; 32]> {
    let (message_id, program_id) =
        common_upload_program(client, gclient::code_from_os(path)?, payload).await?;

    assert!(listener
        .message_processed(message_id.into())
        .await?
        .succeed());

    Ok(program_id)
}

async fn send_message<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
    increase_gas: bool,
) -> Result<Result<T, String>> {
    let encoded_payload = payload.encode();
    let destination = destination.into();

    let gas_limit = client
        .calculate_handle_gas(None, destination, encoded_payload, 0, true)
        .await?
        .min_limit;

    let modified_gas_limit = if increase_gas {
        gas_limit + (gas_limit * 50) / 100
    } else {
        gas_limit
    };

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

#[tokio::test]
async fn battle() -> Result<()> {
    // let address = WSAddress::new("wss://node-workshop.gear.rs", 443);
    // let client = GearApi::init_with(address, "//Alice").await?;
    let client = GearApi::dev().await?.with("//Alice")?;
    let mut listener = client.subscribe().await?;
    // Fund players
    let alice_balance = client.total_balance(client.account_id()).await?;
    let amount = alice_balance / 20;

    for player in PLAYERS {
        client
            .transfer(
                client
                    .get_specific_actor_id(player)
                    .encode()
                    .as_slice()
                    .try_into()
                    .expect("Unexpected invalid `ProgramId`."),
                amount,
            )
            .await?;
    }
    let mut tmg_ids: Vec<ActorId> = Vec::new();
    let mut actor_ids_to_str: BTreeMap<ActorId, &str> = BTreeMap::new();

    // upload tamagotchis
    for player in PLAYERS.iter() {
        let client = client
            .clone()
            .with(player)
            .expect("Unable to change signer.");
        let actor_id = client.get_actor_id();
        actor_ids_to_str.insert(actor_id, player);

        let tmg_id = upload_program(&client, &mut listener, PATHS[0], player.to_string()).await?;
        println!("Tamagotchi `{player}` is initialized.");
        tmg_ids.push(tmg_id.into());
    }

    let client = client
        .clone()
        .with("//Alice")
        .expect("Unable to change signer.");

    // upload battle contract
    let battle_id = upload_program(&client, &mut listener, PATHS[1], "").await?;

    let battle_id_hex = hex::encode(battle_id);
    println!("BATTLE ID {:?}", battle_id_hex);

    // register tamagotchis
    for i in 0..PLAYERS.len() {
        let tmg_id = tmg_ids[i as usize];
        let client = client
            .clone()
            .with(PLAYERS[i])
            .expect("Unable to change signer.");
        assert!(
            Ok(BattleEvent::Registered { tmg_id })
                == send_message(
                    &client,
                    &mut listener,
                    battle_id,
                    BattleAction::Register { tmg_id },
                    true
                )
                .await?
        );
        println!("Tamagotchi {i} is registered.");
    }

    let client = client
        .clone()
        .with("//Alice")
        .expect("Unable to change signer.");
    // start battle
    assert!(
        Ok(BattleEvent::BattleStarted)
            == send_message(
                &client,
                &mut listener,
                battle_id,
                BattleAction::StartBattle,
                true
            )
            .await?
    );

    // read pair_ids for players
    for player in PLAYERS.iter() {
        let player_id = client.get_specific_actor_id(player);
        let pair_ids: Vec<PairId> = client
            .read_state_using_wasm_by_path(
                battle_id.into(),
                "pairs_for_player",
                META_WASM,
                Some(player_id),
            )
            .await?;
        println!(" Pairs {:?} for players {:?} ", pair_ids, player);
    }

    let mut battle_state = BattleState::GameIsOn;

    while battle_state != BattleState::GameIsOver {
        let tmg_ids: Vec<ActorId> = client
            .read_state_using_wasm_by_path(
                battle_id.into(),
                "tmg_ids",
                META_WASM,
                <Option<()>>::None,
            )
            .await?;
        println!("Tmg Ids {:?}", tmg_ids);

        if battle_state == BattleState::WaitNextRound {
            let client = client
                .clone()
                .with("//Alice")
                .expect("Unable to change signer.");
            // start battle
            assert!(
                Ok(BattleEvent::BattleStarted)
                    == send_message(
                        &client,
                        &mut listener,
                        battle_id,
                        BattleAction::StartBattle,
                        true
                    )
                    .await?
            );
        }
        let pair_ids: Vec<PairId> = client
            .read_state_using_wasm_by_path(
                battle_id.into(),
                "pair_ids",
                META_WASM,
                <Option<()>>::None,
            )
            .await?;
        println!("Current Pair Ids {:?}", pair_ids);
        for pair_id in pair_ids.iter() {
            let mut game_is_over = false;
            while game_is_over == false {
                let pair: Pair = client
                    .read_state_using_wasm_by_path(
                        battle_id.into(),
                        "pair",
                        META_WASM,
                        Some(pair_id),
                    )
                    .await?;

                let (power, health): (u16, u16) = client
                    .read_state_using_wasm_by_path(
                        battle_id.into(),
                        "power_and_health",
                        META_WASM,
                        Some(pair.tmg_ids[0]),
                    )
                    .await?;

                println!(
                    "Power {:?} and health {:?} for first tamagotchi",
                    power, health
                );

                let (power, health): (u16, u16) = client
                    .read_state_using_wasm_by_path(
                        battle_id.into(),
                        "power_and_health",
                        META_WASM,
                        Some(pair.tmg_ids[1]),
                    )
                    .await?;

                println!(
                    "Power {:?} and health {:?} for first tamagotchi",
                    power, health
                );

                let current_player: ActorId = client
                    .read_state_using_wasm_by_path(
                        battle_id.into(),
                        "current_turn",
                        META_WASM,
                        Some(pair_id),
                    )
                    .await?;
                let player = actor_ids_to_str.get(&current_player).unwrap();
                println!("current pair {:?} current_player {:?}", pair_id, player);
                let client = client
                    .clone()
                    .with(player)
                    .expect("Unable to change signer.");
                assert!(
                    Ok(BattleEvent::MoveMade)
                        == send_message(
                            &client,
                            &mut listener,
                            battle_id,
                            BattleAction::MakeMove {
                                pair_id: *pair_id,
                                tmg_move: Move::Attack
                            },
                            true,
                        )
                        .await?
                );
                game_is_over = client
                    .read_state_using_wasm_by_path(
                        battle_id.into(),
                        "game_is_over",
                        META_WASM,
                        Some(pair_id),
                    )
                    .await?;
            }
        }
        battle_state = client
            .read_state_using_wasm_by_path(
                battle_id.into(),
                "battle_state",
                META_WASM,
                <Option<()>>::None,
            )
            .await?;
        println!("Battle state {:?}", battle_state);
    }
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
