#![no_std]

pub mod messages;
pub use messages::*;

pub mod asserts;

use ico_io::*;

use gstd::{exec, msg, prelude::*, ActorId};

#[derive(Default)]
struct IcoContract {
    ico_state: IcoState,
    start_price: u128,
    price_increase_step: u128,
    time_increase_step: u128,
    tokens_sold: u128,
    tokens_goal: u128,
    owner: ActorId,
    token_address: ActorId,
    token_holders: BTreeMap<ActorId, u128>,
}

static mut ICO_CONTRACT: Option<IcoContract> = None;

impl IcoContract {
    /// Starts ICO contract
    ///
    /// Requirements:
    /// * Only owner can start ICO
    /// * At least `tokens_goal` tokens need to be minted
    /// * ICO can be started only once
    /// * All arguments must be greater than zero
    ///
    /// Arguments:
    /// * `config`: Consists of `duration`, `start_price`, `tokens_goal`, `price_increase_step` and time_increase_step
    ///
    async fn start_ico(&mut self, config: IcoAction) {
        check_input(&config);
        asserts::owner_message(&self.owner, "start_ico(): Not owner starts ICO");
        assert!(!self.ico_state.ico_started, "start_ico(): Second ICO start");

        if let IcoAction::StartSale {
            duration,
            start_price,
            tokens_goal,
            price_increase_step,
            time_increase_step,
        } = config
        {
            self.start_price = start_price;
            self.tokens_goal = tokens_goal;
            self.price_increase_step = price_increase_step;
            self.time_increase_step = time_increase_step;

            transfer_tokens(
                &self.token_address,
                &self.owner,
                &exec::program_id(),
                self.tokens_goal,
            )
            .await;

            self.ico_state.ico_started = true;
            self.ico_state.duration = duration;
            self.ico_state.start_time = exec::block_timestamp();

            msg::reply(
                IcoEvent::SaleStarted {
                    duration,
                    start_price,
                    tokens_goal,
                    price_increase_step,
                    time_increase_step,
                },
                0,
            )
            .expect("Error in reply");
        }
    }

    /// Purchase of tokens
    ///
    /// Requirements:
    /// * `tokens_cnt` must be greater than zero
    /// * ICO must be in progress (already started and not finished yet)
    /// * `msg::value` must be greater than or equal to `price * tokens_cnt`
    /// * At least `tokens_cnt` tokens available for sale
    ///
    /// Arguments:
    /// * `tokens_cnt`: amount of tokens to purchase
    ///
    pub fn buy_tokens(&mut self, tokens_cnt: u128) {
        let time_now: u64 = exec::block_timestamp();

        assert!(tokens_cnt != 0, "buy_tokens(): Can't buy zero tokens");
        assert!(
            self.ico_state.start_time + self.ico_state.duration >= time_now,
            "buy_tokens(): Duration of the ICO has ended"
        );
        assert!(
            self.get_balance() != 0,
            "buy_tokens(): All tokens have been sold"
        );
        self.check_ico_executing("buy_tokens()");

        assert!(
            tokens_cnt <= self.get_balance(),
            "buy_tokens(): Not enough tokens to sell"
        );

        let current_price = self.get_current_price(time_now);
        let cost = tokens_cnt.checked_mul(current_price).unwrap_or_else(|| {
            panic!(
                "buy_tokens(): Overflowing multiplication: {} * {}",
                tokens_cnt, current_price
            )
        });

        let mut change = 0;
        let amount_sent = msg::value();

        assert!(
            amount_sent >= cost,
            "buy_tokens(): Wrong amount sent, expect {} get {}",
            cost,
            amount_sent
        );

        if amount_sent > cost {
            change = amount_sent - cost;
            msg::send(msg::source(), "", change).expect("Sending error");
        }

        self.token_holders
            .entry(msg::source())
            .and_modify(|balance| *balance += tokens_cnt)
            .or_insert(tokens_cnt);

        self.tokens_sold += tokens_cnt;

        msg::reply(
            IcoEvent::Bought {
                buyer: msg::source(),
                amount: tokens_cnt,
                change,
            },
            0,
        )
        .expect("Error in reply");
    }

    /// Ends ICO contract
    ///
    /// Requirements:
    /// * Only owner can end ICO
    /// * ICO can be ended more only once
    /// * All tokens must be sold or the ICO duration must end
    ///
    async fn end_sale(&mut self) {
        let time_now: u64 = exec::block_timestamp();

        asserts::owner_message(&self.owner, "end_sale()");
        self.check_ico_executing("end_sale()");

        if self.ico_state.start_time + self.ico_state.duration >= time_now
            && self.get_balance() != 0
        {
            panic!(
                "Can't end ICO: tokens left = {}, duration ended = {}",
                self.get_balance(),
                self.ico_state.start_time + self.ico_state.duration < time_now,
            )
        }

        for (id, val) in &self.token_holders {
            transfer_tokens(&self.token_address, &exec::program_id(), id, *val).await;
        }

        let rest_balance = self.get_balance();
        if rest_balance > 0 {
            transfer_tokens(
                &self.token_address,
                &exec::program_id(),
                &self.owner,
                rest_balance,
            )
            .await;

            self.token_holders
                .entry(self.owner)
                .and_modify(|balance| *balance += rest_balance)
                .or_insert(rest_balance);
        }

        self.ico_state.ico_ended = true;
        msg::reply(IcoEvent::SaleEnded, 0).expect("Error in reply");
    }

    fn get_current_price(&self, time_now: u64) -> u128 {
        let amount: u128 = (time_now - self.ico_state.start_time).into();

        self.start_price + self.price_increase_step * (amount / self.time_increase_step)
    }

    fn get_balance(&self) -> u128 {
        self.tokens_goal - self.tokens_sold
    }

    fn check_ico_executing(&self, message: &str) {
        assert!(
            self.ico_state.ico_started,
            "{}: ICO wasn't started",
            message
        );
        assert!(!self.ico_state.ico_ended, "{}: ICO was ended", message);
    }
}

#[gstd::async_main]
async fn main() {
    let action: IcoAction = msg::load().expect("Unable to decode SaleAction");
    let ico: &mut IcoContract = unsafe { ICO_CONTRACT.get_or_insert(Default::default()) };

    match action {
        IcoAction::StartSale { .. } => ico.start_ico(action).await,
        IcoAction::Buy(value) => ico.buy_tokens(value),
        IcoAction::EndSale => ico.end_sale().await,
    }
}

fn check_input(config: &IcoAction) {
    if let IcoAction::StartSale {
        duration,
        start_price,
        tokens_goal,
        price_increase_step,
        time_increase_step,
    } = config
    {
        assert_ne!(*duration, 0, "start_ico(): Init duration is zero");
        assert_ne!(*start_price, 0, "start_ico(): Init start price is zero");
        assert_ne!(*tokens_goal, 0, "start_ico(): Init tokens goal is zero");
        assert_ne!(
            *price_increase_step, 0,
            "start_ico(): Init price increase step is zero"
        );
        assert_ne!(
            *time_increase_step, 0,
            "start_ico(): Init time increase step is zero"
        );
    } else {
        panic!("start_ico(): Wrong init type")
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: IcoInit = msg::load().expect("Unable to decode ICOInit");

    asserts::not_zero_address(&config.token_address, "Init token address");
    asserts::not_zero_address(&config.owner, "Init owner address");

    let ico = IcoContract {
        token_address: config.token_address,
        owner: config.owner,
        ..Default::default()
    };

    unsafe { ICO_CONTRACT = Some(ico) };
}

gstd::metadata! {
    title: "crowdsale_ico",
    init:
        input: IcoInit,
    handle:
        input: IcoAction,
        output: IcoEvent,
    state:
        input: StateIco,
        output: StateIcoReply,
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let time_now: u64 = exec::block_timestamp();

    let state: StateIco = msg::load().expect("failed to decode State");
    let ico: &mut IcoContract = unsafe { ICO_CONTRACT.get_or_insert(IcoContract::default()) };

    let encoded = match state {
        StateIco::CurrentPrice => {
            StateIcoReply::CurrentPrice(ico.get_current_price(time_now)).encode()
        }
        StateIco::TokensLeft => StateIcoReply::TokensLeft(ico.get_balance()).encode(),
        StateIco::BalanceOf(address) => {
            if let Some(val) = ico.token_holders.get(&address) {
                StateIcoReply::BalanceOf {
                    address,
                    balance: *val,
                }
                .encode()
            } else {
                StateIcoReply::BalanceOf {
                    address,
                    balance: 0,
                }
                .encode()
            }
        }
    };

    gstd::util::to_leak_ptr(encoded)
}
