use gstd::{prelude::*, ActorId};
use gtest::{CoreLog, Log, Program, System};
use tamagotchi_battle_io::{
    Battle, BattleAction, BattleError, BattleQuery, BattleQueryReply, BattleReply, Config,
};
use tamagotchi_io::TmgInit;
pub const ADMIN: u64 = 10;
pub trait BattleTestFunc {
    fn tmg_battle(system: &System) -> Program<'_>;
    fn register(&self, tmg_owner: u64, tmg_id: ActorId, error: Option<BattleError>);
    fn start_battle(&self, from: u64, error: Option<BattleError>);
    fn check_in_players(&self, tmg_id: ActorId);
    fn check_in_players_ids(&self, tmg_id: ActorId);
}

impl BattleTestFunc for Program<'_> {
    fn tmg_battle(system: &System) -> Program<'_> {
        let tmg_battle = Program::from_file(
            system,
            "../target/wasm32-unknown-unknown/release/tamagotchi_battle.opt.wasm",
        );

        let result = tmg_battle.send(
            ADMIN,
            Config {
                max_power: 10_000,
                max_range: 10_000,
                min_range: 3_000,
                health: 2_500,
                max_steps_in_round: 5,
                max_participants: 50,
                time_for_move: 60,
                min_gas_amount: 5_000_000_000,
                block_duration_ms: 1_000,
            },
        );
        assert!(!result.main_failed());
        tmg_battle
    }
    fn register(&self, tmg_owner: u64, tmg_id: ActorId, error: Option<BattleError>) {
        let result = self.send(tmg_owner, BattleAction::Register { tmg_id });
        let reply: Result<BattleReply, BattleError>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(tmg_owner, reply.encode())));
        } else {
            reply = Ok(BattleReply::Registered { tmg_id });
            assert!(result.contains(&(tmg_owner, reply.encode())));
        }
    }

    fn start_battle(&self, from: u64, error: Option<BattleError>) {
        let result = self.send(from, BattleAction::StartBattle);
        let reply: Result<BattleReply, BattleError>;
        if let Some(error) = error {
            reply = Err(error);
            assert!(result.contains(&(from, reply.encode())));
        }
    }
    fn check_in_players(&self, tmg_id: ActorId) {
        let reply: BattleQueryReply = self
            .read_state(BattleQuery::GetPlayer { tmg_id })
            .expect("Failed to read state");
        if let BattleQueryReply::Player { player } = reply {
            assert!(player.is_some(), "No player for this tmg");
        } else {
            gstd::panic!("Wrong received reply");
        }
    }
    fn check_in_players_ids(&self, tmg_id: ActorId) {
        let reply: BattleQueryReply = self
            .read_state(BattleQuery::PlayersIds)
            .expect("Failed to read state");
        if let BattleQueryReply::PlayersIds { players_ids } = reply {
            assert!(players_ids.contains(&tmg_id), "Tmg is not in player ids");
        } else {
            gstd::panic!("Wrong received reply");
        }
    }
}

fn upload_tmg(system: &System, players: Vec<u64>) -> Vec<ActorId> {
    let mut tmg_ids: Vec<ActorId> = Vec::new();
    for player in players {
        let tmg = Program::from_file(
            system,
            "../target/wasm32-unknown-unknown/release/tamagotchi.opt.wasm",
        );
        let result = tmg.send(
            player,
            TmgInit {
                name: "".to_string(),
            },
        );
        assert!(!result.main_failed());
        let tmg_id: [u8; 32] = tmg.id().into();
        tmg_ids.push(tmg_id.into());
    }
    tmg_ids
}

//
#[test]
fn make_move() {
    let system = System::new();
    system.init_logger();
    let tmg_battle = Program::tmg_battle(&system);

    // players
    let players = vec![100, 101];

    let tmg_ids = upload_tmg(&system, players.clone());

    for (i, player) in players.into_iter().enumerate() {
        tmg_battle.register(player, tmg_ids[i], None);
        tmg_battle.check_in_players(tmg_ids[i]);
        tmg_battle.check_in_players_ids(tmg_ids[i]);
    }

    // start battle
    tmg_battle.start_battle(ADMIN, None);

    system.spend_blocks(2000);
}
