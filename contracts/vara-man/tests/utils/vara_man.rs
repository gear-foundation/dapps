use super::{ADMIN, VARA_MAN_ID};
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use vara_man_io::{VaraMan as VaraManState, *};

pub trait VaraMan {
    fn vara_man(system: &System) -> Program<'_>;
    fn vara_man_with_config(system: &System, config: Config) -> Program<'_>;
    fn register_player(&self, from: u64, name: &str, error: bool);
    fn start_game(&self, from: u64, level: Level, error: bool);
    fn claim_reward(&self, from: u64, silver_coins: u64, gold_coins: u64, error: bool);
    fn change_status(&self, from: u64, status: Status);
    fn change_config(&self, from: u64, config: Config);
    fn add_admin(&self, from: u64, admin: ActorId);
    fn send_tx(&self, from: u64, action: VaraManAction, error: bool);
    fn get_state(&self) -> VaraManState;
}

impl VaraMan for Program<'_> {
    fn vara_man(system: &System) -> Program<'_> {
        Self::vara_man_with_config(
            system,
            Config {
                gold_coins: 5,
                silver_coins: 20,
                ..Default::default()
            },
        )
    }

    fn vara_man_with_config(system: &System, config: Config) -> Program<'_> {
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

    fn start_game(&self, from: u64, level: Level, error: bool) {
        self.send_tx(from, VaraManAction::StartGame { level }, error);
    }

    fn claim_reward(&self, from: u64, silver_coins: u64, gold_coins: u64, error: bool) {
        self.send_tx(
            from,
            VaraManAction::ClaimReward {
                silver_coins,
                gold_coins,
            },
            error,
        );
    }

    fn change_status(&self, from: u64, status: Status) {
        self.send_tx(from, VaraManAction::ChangeStatus(status), false);
    }

    fn change_config(&self, from: u64, config: Config) {
        self.send_tx(from, VaraManAction::ChangeConfig(config), false);
    }

    fn add_admin(&self, from: u64, admin: ActorId) {
        self.send_tx(from, VaraManAction::AddAdmin(admin), false);
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

        assert_eq!(maybe_error.is_some(), error, "Error: {:#?}", maybe_error);
    }

    fn get_state(&self) -> VaraManState {
        self.read_state().expect("Unexpected invalid state.")
    }
}
