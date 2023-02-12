use gclient::{EventListener, EventProcessor, GearApi, Result, WSAddress};

use battle_io::*;
use fmt::Debug;
use gstd::{prelude::*, ActorId};
use tmg_io::*;
const PATHS: [&str; 2] = [
    "../target/wasm32-unknown-unknown/release/tmg.opt.wasm",
    "../target/wasm32-unknown-unknown/release/battle.opt.wasm",
];
const META_WASM: &str = "../target/wasm32-unknown-unknown/release/battle_state.meta.wasm";
const PLAYERS_LEN: u8 = 3;
const ADDRESS: [u8; 32] =
    hex_literal::hex!("ee141cedb40d043e68f2b9a76992eca562bcd6c4c89870fbba0bcfd44ae9167d");
const TMGS: [&str; 4] = [
    "b0f17182f598aadc2377e3744d7894ad31db06a0b906cce398643671b05e5503",
    "d608b5a451112eb6850bb40562164e0dd0acda112242c961294e80cd79660e58",
    "3a3e09e619556cd27dbaae55f9f343269a97c430a1a592e4c6df27ded4de266c",
    "3a3e09e619556cd27dbaae55f9f343269a97c430a1a592e4c6df27ded4de266c",
];

const BATTLE_ADDRESS: [u8; 32] =
    hex_literal::hex!("95a93fefed36f7efba9d03211df635b680baf99c62c3cb9e1ceb16f8b87ff33e");
fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
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
        .send_message(destination, payload, modified_gas_limit, 0)
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
    let address = WSAddress::new("wss://node-workshop.gear.rs", 443);
    let client = GearApi::init_with(address, "//Alice").await?;
    //let client = GearApi::dev().await?.with("//Alice")?;
    let mut listener = client.subscribe().await?;

    let mut players: Vec<ActorId> = Vec::new();

    // upload tamagotchis
    for i in 0..PLAYERS_LEN {
        let tmg_id =
            upload_program(&client, &mut listener, PATHS[0], "Tamagotchi".to_string()).await?;
        println!("Tamagotchi `{i}` is initialized.");
        players.push(tmg_id.into());
    }

    // upload battle contract
    let battle_id = upload_program(&client, &mut listener, PATHS[1], "").await?;

    let battle_id_hex = hex::encode(battle_id);
    println!("BATTLE ID {:?}", battle_id_hex);

    // register tamagotchis
    for i in 0..PLAYERS_LEN {
        let tmg_id = players[i as usize];
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

        let player: Player = client
            .read_state_using_wasm_by_path(battle_id.into(), "player", META_WASM, Some(tmg_id))
            .await?;

        println!("Tamagotchi {i} is registered.");
        println!(" {player:?} ");
    }

    // start battle
    assert!(
        Ok(BattleEvent::GameStarted)
            == send_message(
                &client,
                &mut listener,
                battle_id,
                BattleAction::StartBattle,
                true
            )
            .await?
    );

    // first round
    println!("First round");
    let round: Round = client
        .read_state_using_wasm_by_path(battle_id.into(), "round", META_WASM, <Option<()>>::None)
        .await?;
    println!(" {round:?} ");

    let mut player_0 = round.tmg_ids[0];
    let mut player_1 = round.tmg_ids[1];

    while (true) {
        let (power_0, health_0) = get_power_and_health(&client, battle_id, &player_0).await;
        let (power_1, health_1) = get_power_and_health(&client, battle_id, &player_1).await;

        println!("-----");
        println!("tamagotchi 0 power {power_0} and health {health_0}");
        println!("tamagotchi 1 power {power_1} and health {health_1}");
        println!("-----");
        let move_0 = if health_0 < 1000 && power_1 > power_0 {
            Move::Defence
        } else {
            Move::Attack
        };

        let move_1 = if health_1 < 1000 && power_0 > power_1 {
            Move::Defence
        } else {
            Move::Attack
        };

        assert!(
            Ok(BattleEvent::MoveMade)
                == send_message(
                    &client,
                    &mut listener,
                    battle_id,
                    BattleAction::MakeMove(move_0),
                    true,
                )
                .await?
        );
        assert!(
            Ok(BattleEvent::MoveMade)
                == send_message(
                    &client,
                    &mut listener,
                    battle_id,
                    BattleAction::MakeMove(move_1),
                    true,
                )
                .await?
        );

        let state: BattleState = client
            .read_state_using_wasm_by_path(
                battle_id.into(),
                "battle_state",
                META_WASM,
                <Option<()>>::None,
            )
            .await?;

        if state == BattleState::WaitNextRound {
            println!("-----");
            println!("Next round");

            assert!(
                Ok(BattleEvent::NewRound)
                    == send_message(
                        &client,
                        &mut listener,
                        battle_id,
                        BattleAction::StartNewRound,
                        true,
                    )
                    .await?
            );

            let round: Round = client
                .read_state_using_wasm_by_path(
                    battle_id.into(),
                    "round",
                    META_WASM,
                    <Option<()>>::None,
                )
                .await?;
            player_0 = round.tmg_ids[0];
            player_1 = round.tmg_ids[1];
            println!(" {round:?} ");
        }
        if state == BattleState::GameIsOver {
            break;
        }
    }

    Ok(())
}

#[tokio::test]
async fn add_players() -> Result<()> {
    let address = WSAddress::new("wss://node-workshop.gear.rs", 443);
    let client = GearApi::init_with(address, "//Alice").await?;
    //let client = GearApi::dev().await?.with("//Alice")?;
    let mut listener = client.subscribe().await?;

    let mut players: Vec<ActorId> = Vec::new();

    // upload tamagotchis
    for i in 0..PLAYERS_LEN {
        let tmg_id =
            upload_program(&client, &mut listener, PATHS[0], "Tamagotchi".to_string()).await?;
        println!("Tamagotchi `{i}` is initialized.");
        players.push(tmg_id.into());
    }

    let tmg_hex =
        hex_literal::hex!("b0f17182f598aadc2377e3744d7894ad31db06a0b906cce398643671b05e5503");

    assert!(
        Ok(TmgEvent::Transfer(tmg_hex.into()))
            == send_message(
                &client,
                &mut listener,
                players[0].into(),
                TmgAction::Transfer(tmg_hex.into()),
                true,
            )
            .await?
    );

    let tmg_hex =
        hex_literal::hex!("d608b5a451112eb6850bb40562164e0dd0acda112242c961294e80cd79660e58");

    assert!(
        Ok(TmgEvent::Transfer(tmg_hex.into()))
            == send_message(
                &client,
                &mut listener,
                players[1].into(),
                TmgAction::Transfer(tmg_hex.into()),
                true,
            )
            .await?
    );

    let tmg_hex =
        hex_literal::hex!("3a3e09e619556cd27dbaae55f9f343269a97c430a1a592e4c6df27ded4de266c");

    assert!(
        Ok(TmgEvent::Transfer(tmg_hex.into()))
            == send_message(
                &client,
                &mut listener,
                players[2].into(),
                TmgAction::Transfer(tmg_hex.into()),
                true,
            )
            .await?
    );

    let tmg_hex =
        hex_literal::hex!("18bbd04c80d185bc0a73d9186138dc26c0297c803580568662d15828e2dede0c");

    assert!(
        Ok(TmgEvent::Transfer(tmg_hex.into()))
            == send_message(
                &client,
                &mut listener,
                players[3].into(),
                TmgAction::Transfer(tmg_hex.into()),
                true,
            )
            .await?
    );

    // register tamagotchis
    for i in 0..PLAYERS_LEN {
        let tmg_id = players[i as usize];
        assert!(
            Ok(BattleEvent::Registered { tmg_id })
                == send_message(
                    &client,
                    &mut listener,
                    BATTLE_ADDRESS.into(),
                    BattleAction::Register { tmg_id },
                    true
                )
                .await?
        );

        let player: Player = client
            .read_state_using_wasm_by_path(BATTLE_ADDRESS.into(), "player", META_WASM, Some(tmg_id))
            .await?;

        println!("Tamagotchi {i} is registered.");
        println!(" {player:?} ");
    }

    Ok(())
}
async fn get_power_and_health(
    client: &GearApi,
    battle_id: [u8; 32],
    tmg_id: &ActorId,
) -> (u16, u16) {
    let (power, health): (u16, u16) = client
        .read_state_using_wasm_by_path(
            battle_id.into(),
            "power_and_health",
            META_WASM,
            Some(*tmg_id),
        )
        .await
        .expect("Error during reading state");
    (power, health)
}
