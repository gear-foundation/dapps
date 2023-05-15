use super::{ADMIN, VARA_MAN_ID};
use gstd::prelude::*;
use gtest::{Program, System};
use vara_man_io::{
    Config, GameSeed, Level, Status, VaraMan as VaraManState, VaraManAction, VaraManEvent,
    VaraManInit,
};

pub trait VaraMan {
    fn vara_man(system: &System) -> Program;
    fn vara_man_with_config(system: &System, config: Config) -> Program;
    fn register_player(&self, from: u64, name: &str, error: bool);
    fn start_game(&self, from: u64, level: Level, seed: GameSeed, error: bool);
    fn claim_reward(
        &self,
        from: u64,
        game_id: u64,
        silver_coins: u64,
        gold_coins: u64,
        error: bool,
    );
    fn change_status(&self, status: Status);
    fn change_config(&self, config: Config);
    fn send_tx(&self, from: u64, action: VaraManAction, error: bool);
    fn get_state(&self) -> VaraManState;
}

impl VaraMan for Program<'_> {
    fn vara_man(system: &System) -> Program {
        Self::vara_man_with_config(
            system,
            Config {
                operator: ADMIN.into(),
                reward_token_id: ADMIN.into(),
                gold_coins: 5,
                silver_coins: 20,
                ..Default::default()
            },
        )
    }

    fn vara_man_with_config(system: &System, config: Config) -> Program {
        let vara_man = Program::current_with_id(system, VARA_MAN_ID);
        assert!(!vara_man.send(ADMIN, VaraManInit { config }).main_failed());

        vara_man
    }

    fn register_player(&self, from: u64, name: &str, error: bool) {
        self.send_tx(
            from,
            VaraManAction::RegisterPlayer {
                name: name.to_owned(),
            },
            error,
        );
    }

    fn start_game(&self, from: u64, level: Level, seed: GameSeed, error: bool) {
        self.send_tx(from, VaraManAction::StartGame { level, seed }, error);
    }

    fn claim_reward(
        &self,
        from: u64,
        game_id: u64,
        silver_coins: u64,
        gold_coins: u64,
        error: bool,
    ) {
        self.send_tx(
            from,
            VaraManAction::ClaimReward {
                game_id,
                silver_coins,
                gold_coins,
            },
            error,
        );
    }

    fn change_status(&self, status: Status) {
        self.send_tx(ADMIN, VaraManAction::ChangeStatus(status), false);
    }

    fn change_config(&self, config: Config) {
        self.send_tx(ADMIN, VaraManAction::ChangeConfig(config), false);
    }

    fn send_tx(&self, from: u64, action: VaraManAction, error: bool) {
        let result = self.send(from, action);
        assert!(!result.main_failed());

        let maybe_error = result.log().iter().find_map(|log| {
            let mut payload = log.payload();
            if let Ok(VaraManEvent::Error(error)) = VaraManEvent::decode(&mut payload) {
                Some(error)
            } else {
                None
            }
        });

        assert_eq!(maybe_error.is_some(), error);
    }

    fn get_state(&self) -> VaraManState {
        self.read_state().expect("Unexpected invalid state.")
    }
}
