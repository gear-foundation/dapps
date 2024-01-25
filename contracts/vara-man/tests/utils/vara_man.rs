use super::{ADMIN, VARA_MAN_ID};
use fungible_token_io::{FTAction, InitConfig};
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use vara_man_io::{VaraMan as VaraManState, *};

pub trait VaraMan {
    fn vara_man(system: &System) -> Program<'_>;
    fn vara_man_with_config(system: &System, config: Config) -> Program<'_>;
    fn register_player(&self, from: u64, name: &str, error: Option<VaraManError>);
    fn start_game(&self, from: u64, level: Level, error: Option<VaraManError>);
    fn claim_reward(
        &self,
        from: u64,
        silver_coins: u64,
        gold_coins: u64,
        error: Option<VaraManError>,
    );
    fn change_status(&self, from: u64, status: Status);
    fn change_config(&self, from: u64, config: Config);
    fn add_admin(&self, from: u64, admin: ActorId);
    fn send_tx(&self, from: u64, action: VaraManAction, error: Option<VaraManError>);
    fn get_state(&self) -> Option<VaraManState>;
}

impl VaraMan for Program<'_> {
    fn vara_man(system: &System) -> Program<'_> {
        Self::vara_man_with_config(
            system,
            Config {
                gold_coins: 5,
                silver_coins: 20,
                number_of_lives: 3,
                ..Default::default()
            },
        )
    }

    fn vara_man_with_config(system: &System, config: Config) -> Program<'_> {
        let vara_man = Program::current_with_id(system, VARA_MAN_ID);
        assert!(!vara_man.send(ADMIN, VaraManInit { config }).main_failed());

        vara_man
    }

    fn register_player(&self, from: u64, name: &str, error: Option<VaraManError>) {
        self.send_tx(
            from,
            VaraManAction::RegisterPlayer {
                name: name.to_owned(),
            },
            error,
        );
    }

    fn start_game(&self, from: u64, level: Level, error: Option<VaraManError>) {
        self.send_tx(from, VaraManAction::StartGame { level }, error);
    }

    fn claim_reward(
        &self,
        from: u64,
        silver_coins: u64,
        gold_coins: u64,
        error: Option<VaraManError>,
    ) {
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
        self.send_tx(from, VaraManAction::ChangeStatus(status), None);
    }

    fn change_config(&self, from: u64, config: Config) {
        self.send_tx(from, VaraManAction::ChangeConfig(config), None);
    }

    fn add_admin(&self, from: u64, admin: ActorId) {
        self.send_tx(from, VaraManAction::AddAdmin(admin), None);
    }

    fn send_tx(&self, from: u64, action: VaraManAction, error: Option<VaraManError>) {
        let result = self.send(from, action);
        assert!(!result.main_failed());
        let res = &result.decoded_log::<Result<VaraManEvent, VaraManError>>();
        println!("RESULT: {:?}", res);
        if let Some(error) = error {
            assert!(result.contains(&(from, Err::<VaraManEvent, VaraManError>(error).encode())));
        }
    }

    fn get_state(&self) -> Option<VaraManState> {
        let reply = self
            .read_state(StateQuery::All)
            .expect("Unexpected invalid state.");
        if let StateReply::All(state) = reply {
            Some(state)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn init_mint_transfer_fungible_token(sys: &System, from: u64, to: u64) -> Program<'_> {
    sys.init_logger();
    let ft = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/debug/fungible_token.opt.wasm",
    );

    let res = ft.send(
        from,
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            decimals: 18,
        },
    );

    assert!(!res.main_failed());

    let res = ft.send(from, FTAction::Mint(100_000_000_000_000));
    assert!(!res.main_failed());

    let res = ft.send(
        from,
        FTAction::Transfer {
            from: from.into(),
            to: to.into(),
            amount: 100_000_000_000_000,
        },
    );
    assert!(!res.main_failed());

    ft
}
