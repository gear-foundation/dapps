use battleship_io::{
    BattleshipAction, BattleshipInit, BattleshipState, Config, Entity, Ships, StateQuery,
    StateReply,
};
use gclient::{EventListener, EventProcessor, GearApi, Result};
use gear_core::ids::ProgramId;
use gstd::Encode;

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
            gclient::now_micros().to_le_bytes(),
            payload,
            gas_limit,
            0,
        )
        .await?;

    Ok((message_id.into(), program_id.into()))
}

#[tokio::test]
async fn gclient_start_game_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.
                                               // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let bot_actor_id = upload_program(
        &api,
        &mut listener,
        "../target/wasm32-unknown-unknown/debug/battleship_bot.opt.wasm",
        0,
    )
    .await?;

    let init_battleship = BattleshipInit {
        bot_address: bot_actor_id.into(),
        config: Config {
            gas_for_start: 3_000_000_000,
            gas_for_turn: 3_000_000_000,
        },
    }
    .encode();

    let path = "../target/wasm32-unknown-unknown/debug/battleship.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path)?,
            init_battleship.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            init_battleship,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    let ships = Ships {
        ship_1: vec![19],
        ship_2: vec![0, 1, 2],
        ship_3: vec![4, 9],
        ship_4: vec![16, 21],
    };

    let start_payload = BattleshipAction::StartGame { ships };

    let gas_info = api
        .calculate_handle_gas(None, program_id, start_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(program_id, start_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_turn_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let mut listener = api.subscribe().await?; // Subscribing for events.
                                               // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let bot_actor_id = upload_program(
        &api,
        &mut listener,
        "../target/wasm32-unknown-unknown/debug/battleship_bot.opt.wasm",
        0,
    )
    .await?;

    let init_battleship = BattleshipInit {
        bot_address: bot_actor_id.into(),
        config: Config {
            gas_for_start: 3_000_000_000,
            gas_for_turn: 3_000_000_000,
        },
    }
    .encode();

    let path = "../target/wasm32-unknown-unknown/debug/battleship.opt.wasm";
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path)?,
            init_battleship.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            init_battleship,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    for _i in 1..3 {
        let ships = Ships {
            ship_1: vec![19],
            ship_2: vec![0, 1, 2],
            ship_3: vec![4, 9],
            ship_4: vec![16, 21],
        };
        let start_payload = BattleshipAction::StartGame { ships };

        let gas_info = api
            .calculate_handle_gas(None, program_id, start_payload.encode(), 0, true)
            .await?;

        let (message_id, _) = api
            .send_message(program_id, start_payload, gas_info.min_limit, 0)
            .await?;

        assert!(listener.message_processed(message_id).await?.succeed());
        assert!(listener.blocks_running().await?);
        let steps: Vec<u8> = (0..25).collect();
        for step in steps {
            let state = get_all_state(&api, &program_id)
                .await
                .expect("Unexpected invalid state.");
            if (state.games[0].1.bot_board[step as usize] == Entity::Empty
                || state.games[0].1.bot_board[step as usize] == Entity::Ship)
                && !state.games[0].1.game_over
            {
                let turn_payload = BattleshipAction::Turn { step };
                let gas_info = api
                    .calculate_handle_gas(None, program_id, turn_payload.encode(), 0, true)
                    .await?;
                let (message_id, _) = api
                    .send_message(program_id, turn_payload, gas_info.min_limit, 0)
                    .await?;
                assert!(listener.message_processed(message_id).await?.succeed());
                assert!(listener.blocks_running().await?);
            }
        }
    }

    Ok(())
}

pub async fn get_all_state(api: &GearApi, program_id: &ProgramId) -> Option<BattleshipState> {
    let reply = api
        .read_state(*program_id, StateQuery::All.encode())
        .await
        .expect("Unexpected invalid reply.");
    if let StateReply::All(state) = reply {
        Some(state)
    } else {
        None
    }
}
