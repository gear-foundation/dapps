use super::{ADMIN, VARA_MAN_ID};
use fungible_token_io::{FTAction, InitConfig};
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use vara_man_io::*;

pub trait VaraMan {
    fn vara_man(system: &System) -> Program<'_>;
    fn vara_man_with_config(system: &System, config: Config) -> Program<'_>;
    fn record_tournament_result(
        &self,
        from: u64,
        time: u128,
        gold_coins: u128,
        silver_coins: u128,
        error: Option<VaraManError>,
    );
    fn finish_single_game(
        &self,
        from: u64,
        gold_coins: u128,
        silver_coins: u128,
        error: Option<VaraManError>,
    );
    fn leave_game(&self, from: u64, error: Option<VaraManError>);
    #[allow(clippy::too_many_arguments)]
    fn create_tournament(
        &self,
        from: u64,
        tournament_name: String,
        name: String,
        level: Level,
        duration_ms: u32,
        value: u128,
        error: Option<VaraManError>,
    );
    fn register(
        &self,
        from: u64,
        admin: ActorId,
        name: String,
        value: u128,
        error: Option<VaraManError>,
    );
    fn cancel_register(&self, from: u64, error: Option<VaraManError>);
    fn change_status(&self, from: u64, status: Status);
    fn change_config(&self, from: u64, config: Config);
    fn start_tournament(&self, from: u64, error: Option<VaraManError>);
    fn add_admin(&self, from: u64, admin: ActorId);
    fn send_tx(&self, from: u64, action: VaraManAction, error: Option<VaraManError>);
    fn send_tx_with_value(
        &self,
        from: u64,
        action: VaraManAction,
        value: u128,
        error: Option<VaraManError>,
    );
    fn get_state(&self) -> Option<VaraManState>;
}

impl VaraMan for Program<'_> {
    fn vara_man(system: &System) -> Program<'_> {
        Self::vara_man_with_config(
            system,
            Config {
                one_point_in_value: 10_000_000_000_000,
                points_per_gold_coin_easy: 5,
                points_per_silver_coin_easy: 1,
                points_per_gold_coin_medium: 8,
                points_per_silver_coin_medium: 2,
                points_per_gold_coin_hard: 10,
                points_per_silver_coin_hard: 3,
                gas_for_finish_tournament: 10_000_000_000,
                time_for_single_round: 180_000,
            },
        )
    }

    fn vara_man_with_config(system: &System, config: Config) -> Program<'_> {
        let vara_man = Program::current_with_id(system, VARA_MAN_ID);
        assert!(!vara_man.send(ADMIN, VaraManInit { config }).main_failed());
        vara_man
    }

    fn finish_single_game(
        &self,
        from: u64,
        gold_coins: u128,
        silver_coins: u128,
        error: Option<VaraManError>,
    ) {
        self.send_tx(
            from,
            VaraManAction::FinishSingleGame {
                gold_coins,
                silver_coins,
                level: Level::Easy,
            },
            error,
        );
    }
    fn record_tournament_result(
        &self,
        from: u64,
        time: u128,
        gold_coins: u128,
        silver_coins: u128,
        error: Option<VaraManError>,
    ) {
        self.send_tx(
            from,
            VaraManAction::RecordTournamentResult {
                time,
                gold_coins,
                silver_coins,
            },
            error,
        );
    }
    fn leave_game(&self, from: u64, error: Option<VaraManError>) {
        self.send_tx(from, VaraManAction::LeaveGame, error);
    }
    fn create_tournament(
        &self,
        from: u64,
        tournament_name: String,
        name: String,
        level: Level,
        duration_ms: u32,
        value: u128,
        error: Option<VaraManError>,
    ) {
        self.send_tx_with_value(
            from,
            VaraManAction::CreateNewTournament {
                tournament_name,
                name,
                level,
                duration_ms,
            },
            value,
            error,
        );
    }
    fn register(
        &self,
        from: u64,
        admin_id: ActorId,
        name: String,
        value: u128,
        error: Option<VaraManError>,
    ) {
        self.send_tx_with_value(
            from,
            VaraManAction::RegisterForTournament { admin_id, name },
            value,
            error,
        );
    }
    fn cancel_register(&self, from: u64, error: Option<VaraManError>) {
        self.send_tx(from, VaraManAction::CancelRegister, error);
    }
    fn start_tournament(&self, from: u64, error: Option<VaraManError>) {
        self.send_tx(from, VaraManAction::StartTournament, error);
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
        if let Some(error) = error {
            assert!(result.contains(&(from, Err::<VaraManEvent, VaraManError>(error).encode())));
        }
    }
    fn send_tx_with_value(
        &self,
        from: u64,
        action: VaraManAction,
        value: u128,
        error: Option<VaraManError>,
    ) {
        let result = self.send_with_value(from, action, value);
        assert!(!result.main_failed());
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
