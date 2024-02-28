use fmt::Debug;
use gclient::{EventListener, EventProcessor, GearApi, Result};
use gear_core::ids::ProgramId;
use gstd::{collections::BTreeMap, prelude::*, ActorId};
use syndote_io::*;

const PATHS: [&str; 2] = [
    "../target/wasm32-unknown-unknown/release/syndote_player.opt.wasm",
    "../target/wasm32-unknown-unknown/release/syndote.opt.wasm",
];

const PLAYERS: [&str; 4] = ["//John", "//Mike", "//Dan", "//Bot"];

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

pub async fn send_message<T: Decode>(
    client: &GearApi,
    listener: &mut EventListener,
    destination: [u8; 32],
    payload: impl Encode + Debug,
    gas_limit: u64,
    _increase_gas: bool,
) -> Result<Result<T, String>> {
    let destination = destination.into();

    println!("Sending a payload: `{payload:?}`.");

    let (message_id, _) = client
        .send_message(destination, payload, gas_limit, 0)
        .await?;

    println!("Sending completed.");

    let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

    Ok(match raw_reply {
        Ok(raw_reply) => Ok(T::decode(&mut raw_reply.as_slice())?),
        Err(error) => Err(error),
    })
}

pub async fn send_balances(client: &GearApi) -> Result<BTreeMap<ActorId, String>> {
    // 100 Vara
    let amount = 100_000_000_000_000;
    let mut actor_id_to_name = BTreeMap::new();
    for player in PLAYERS {
        let actor_id = client.get_specific_actor_id(player);
        actor_id_to_name.insert(actor_id, player.to_string());
        client
            .transfer(
                actor_id
                    .encode()
                    .as_slice()
                    .try_into()
                    .expect("Unexpected invalid `ProgramId`."),
                amount,
            )
            .await?;
    }
    Ok(actor_id_to_name)
}

pub async fn upload_and_register_players(
    client: &GearApi,
    listener: &mut EventListener,
    admin_id: ActorId,
    game_id: ProgramId,
) -> Result<()> {
    // upload 4 strategies
    let strategy_code = gclient::code_from_os(PATHS[0])?;
    let codes = [
        strategy_code.clone(),
        strategy_code.clone(),
        strategy_code.clone(),
        strategy_code.clone(),
    ];
    let codes_len = codes.len();

    // Sending batch.
    let mut args = Vec::new();
    for i in 0..4 {
        let mut salt = gclient::now_micros().to_le_bytes();
        salt[0] = i;
        args.push((strategy_code.clone(), salt, "", 5_000_000_000, 0))
    }

    let (ex_res, _) = client.upload_program_bytes_batch(args).await?;

    // Ids of initial messages.
    let res: Vec<_> = ex_res
        .into_iter()
        .filter_map(|v| v.ok().map(|(mid, pid)| (mid, pid)))
        .collect();
    let (mids, pids): (Vec<_>, Vec<_>) = res.into_iter().unzip();

    // Checking that all upload program calls succeed in batch.
    assert_eq!(codes_len, mids.len());

    // Checking that all batch got processed.
    assert_eq!(
        codes_len,
        listener.message_processed_batch(mids).await?.len(),
    );

    for (i, player) in PLAYERS.iter().enumerate() {
        let exp_reply: Result<GameReply, GameError> = Ok(GameReply::StrategyRegistered);
        let pid: [u8; 32] = pids[i].into();
        let client = client.clone().with(player)?;
        assert_eq!(
            Ok(exp_reply),
            send_message(
                &client,
                listener,
                game_id.into(),
                GameAction::Register {
                    admin_id,
                    strategy_id: pid.into(),
                },
                730_000_000_000,
                false
            )
            .await?
        );
    }

    Ok(())
}

pub async fn make_reservation(
    client: &GearApi,
    listener: &mut EventListener,
    game_id: ProgramId,
    number_of_reservations: u8,
    admin_id: ActorId,
) -> Result<()> {
    let messages = vec![
        (
            game_id,
            GameAction::MakeReservation { admin_id }.encode(),
            730_000_000_000,
            0
        );
        number_of_reservations as usize
    ];

    let (messages, _hash) = client.send_message_bytes_batch(messages).await?;

    let (message_id, _hash) = messages.last().unwrap().as_ref().unwrap();

    assert!(listener.message_processed(*message_id).await?.succeed());

    Ok(())
}

pub async fn upload_syndote(
    client: &GearApi,
    listener: &mut EventListener,
    config: Config,
) -> Result<ProgramId> {
    let (message_id, program_id, _) = client
        .upload_program_by_path(
            PATHS[1],
            gclient::now_micros().to_le_bytes(),
            config,
            10_000_000_000,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    Ok(program_id)
}

pub async fn get_game_session(
    client: &GearApi,
    game_id: ProgramId,
    admin_id: ActorId,
) -> Result<GameState> {
    let reply: StateReply = client
        .read_state(game_id, StateQuery::GetGameSession { admin_id }.encode())
        .await?;
    if let StateReply::GameSession {
        game_session: Some(game_session),
    } = reply
    {
        Ok(game_session)
    } else {
        std::panic!("Wrong received reply or game does not exist")
    }
}

pub async fn get_owner_id(
    client: &GearApi,
    game_id: ProgramId,
    admin_id: ActorId,
    strategy_id: ActorId,
) -> Result<ActorId> {
    let reply: StateReply = client
        .read_state(
            game_id,
            StateQuery::GetOwnerId {
                admin_id,
                strategy_id,
            }
            .encode(),
        )
        .await?;
    if let StateReply::OwnerId {
        owner_id: Some(owner_id),
    } = reply
    {
        Ok(owner_id)
    } else {
        std::panic!("Wrong received reply or player does not exist")
    }
}

pub async fn get_player_info(
    client: &GearApi,
    game_id: ProgramId,
    admin_id: ActorId,
    account_id: ActorId,
) -> Result<PlayerInfo> {
    let reply: StateReply = client
        .read_state(
            game_id,
            StateQuery::GetPlayerInfo {
                admin_id,
                account_id,
            }
            .encode(),
        )
        .await?;
    if let StateReply::PlayerInfo {
        player_info: Some(player_info),
    } = reply
    {
        Ok(player_info)
    } else {
        std::panic!("Wrong received reply or player does not exist")
    }
}
