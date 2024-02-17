use fungible_token_io::{FTAction, InitConfig, IoFungibleToken};
use gstd::Encode;
use gtest::{Program, System};
use varatube_io::{
    Actions, Config, Error, Period, Price, Reply, StateQuery, StateReply, SubscriberData,
};

pub trait FTokenTestFuncs {
    fn ftoken(
        system: &System,
        from: u64,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Program<'_>;
    fn mint(&self, from: u64, amount: u128);
    fn check_balance(&self, account: u64, expected_amount: u128);
    fn approve(&self, from: u64, approved_account: [u8; 32], amount: u128);
}

impl FTokenTestFuncs for Program<'_> {
    fn ftoken(
        system: &System,
        from: u64,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Program<'_> {
        let ftoken = Program::from_file(
            system,
            "../target/wasm32-unknown-unknown/debug/fungible_token.opt.wasm",
        );

        let res = ftoken.send(
            from,
            InitConfig {
                name,
                symbol,
                decimals,
            },
        );
        assert!(!res.main_failed());
        ftoken
    }

    fn mint(&self, from: u64, amount: u128) {
        let payload = FTAction::Mint(amount);

        let res = self.send(from, payload);
        assert!(!res.main_failed());
    }

    fn approve(&self, from: u64, approved_account: [u8; 32], amount: u128) {
        let payload = FTAction::Approve {
            to: approved_account.into(),
            amount,
        };

        let res = self.send(from, payload);
        assert!(!res.main_failed());
    }

    fn check_balance(&self, account: u64, expected_amount: u128) {
        let state: IoFungibleToken = self.read_state(false).expect("Unable to read token state");
        let balance = if let Some((_, balance)) = state
            .balances
            .into_iter()
            .find(|(id, _balance)| *id == account.into())
        {
            balance
        } else {
            0
        };

        assert_eq!(balance, expected_amount, "Error in balances");
    }
}

pub trait VaratubeTestFuncs {
    fn varatube(system: &System, from: u64, config: Config) -> Program<'_>;
    fn register_subscription(
        &self,
        from: u64,
        currency_id: [u8; 32],
        period: Period,
        with_renewal: bool,
        error: Option<Error>,
    );
    fn update_subscription(&self, from: u64, subscriber: u64, error: Option<Error>);
    fn cancel_subscription(&self, from: u64, error: Option<Error>);
    fn add_token_data(&self, from: u64, currency_id: [u8; 32], price: Price, error: Option<Error>);
    fn get_subscriber_data(&self, subscriber: u64) -> Option<SubscriberData>;
}

impl VaratubeTestFuncs for Program<'_> {
    fn varatube(system: &System, from: u64, config: Config) -> Program<'_> {
        let varatube = Program::current(system);
        let res = varatube.send(from, config);
        assert!(!res.main_failed());
        varatube
    }
    fn register_subscription(
        &self,
        from: u64,
        currency_id: [u8; 32],
        period: Period,
        with_renewal: bool,
        error: Option<Error>,
    ) {
        let result = self.send(
            from,
            Actions::RegisterSubscription {
                currency_id: currency_id.into(),
                period,
                with_renewal,
            },
        );
        if let Some(error) = error {
            assert!(result.contains(&(from, error.encode())));
        } else {
            let expected_reply: Result<Reply, Error> = Ok(Reply::SubscriptionRegistered);
            assert!(result.contains(&(from, expected_reply.encode())));
        }
    }

    fn update_subscription(&self, from: u64, subscriber: u64, error: Option<Error>) {
        let result = self.send(
            from,
            Actions::UpdateSubscription {
                subscriber: subscriber.into(),
            },
        );
        if let Some(error) = error {
            assert!(result.contains(&(from, error.encode())));
        } else {
            let expected_reply: Result<Reply, Error> = Ok(Reply::SubscriptionUpdated);
            assert!(result.contains(&(from, expected_reply.encode())));
        }
    }

    fn cancel_subscription(&self, from: u64, error: Option<Error>) {
        let result = self.send(from, Actions::CancelSubscription);
        if let Some(error) = error {
            assert!(result.contains(&(from, error.encode())));
        } else {
            let expected_reply: Result<Reply, Error> = Ok(Reply::SubscriptionCancelled);
            assert!(result.contains(&(from, expected_reply.encode())));
        }
    }

    fn add_token_data(&self, from: u64, currency_id: [u8; 32], price: Price, error: Option<Error>) {
        let result = self.send(
            from,
            Actions::AddTokenData {
                token_id: currency_id.into(),
                price,
            },
        );
        if let Some(error) = error {
            assert!(result.contains(&(from, error.encode())));
        } else {
            let expected_reply: Result<Reply, Error> = Ok(Reply::PaymentAdded);
            assert!(result.contains(&(from, expected_reply.encode())));
        }
    }

    fn get_subscriber_data(&self, subscriber: u64) -> Option<SubscriberData> {
        let state: StateReply = self
            .read_state(StateQuery::GetSubscriber(subscriber.into()))
            .expect("Unable to read varatube state");
        if let StateReply::SubscriberData(subscriber_data) = state {
            subscriber_data
        } else {
            panic!("Wrong received reply");
        }
    }
}
