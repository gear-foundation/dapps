#![no_std]
#![allow(static_mut_refs)]

use extended_vft_client::vft::io as vft_io;
use sails_rs::calls::ActionIo;
use sails_rs::{
    collections::HashMap,
    gstd::{exec, msg},
    prelude::*,
};
mod utils;
use utils::*;

static mut STORAGE: Option<Storage> = None;

#[derive(Default, Clone, Debug)]

pub struct Storage {
    pub admin: ActorId,
    pub reserve_a: U256,
    pub reserve_b: U256,
    pub total_liquidity: U256,
    pub liquidity_providers: HashMap<ActorId, U256>,
    pub token_a: ActorId,
    pub token_b: ActorId,
    pub dns_info: Option<(ActorId, String)>,
}

struct DexService(());

impl DexService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Dex is not initialized") }
    }
    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Dex is not initialized") }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    AddedLiquidity {
        sender: ActorId,
        amount_a: U256,
        amount_b: U256,
        liquidity: U256,
    },
    RemovedLiquidity {
        sender: ActorId,
        amount_a: U256,
        amount_b: U256,
        to: ActorId,
    },
    Swap {
        kind: SwapKind,
        sender: ActorId,
        in_amount: U256,
        out_amount: U256,
        to: ActorId,
    },
    Sync {
        reserve_a: U256,
        reserve_b: U256,
    },
    Killed {
        inheritor: ActorId,
    },
}

#[sails_rs::service(events = Event)]
impl DexService {
    async fn init(
        token_a: ActorId,
        token_b: ActorId,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                admin: msg::source(),
                reserve_a: U256::zero(),
                reserve_b: U256::zero(),
                total_liquidity: U256::zero(),
                liquidity_providers: HashMap::new(),
                token_a,
                token_b,
                dns_info: dns_id_and_name.clone(),
            });
        }

        if let Some((id, name)) = dns_id_and_name {
            let request = [
                "Dns".encode(),
                "AddNewProgram".to_string().encode(),
                (name, exec::program_id()).encode(),
            ]
            .concat();

            msg::send_bytes_with_gas_for_reply(id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }

        Self(())
    }

    /// Adds liquidity to the pool.
    /// Transfers token amounts `amount_a` and `amount_b` from the user to the DEX contract.
    /// If the pool is empty, initializes the reserves and calculates initial liquidity.
    /// If the pool is already initialized, calculates liquidity proportionally to the existing reserves.
    /// Updates the user's liquidity balance and reserves.
    pub async fn add_liquidity(&mut self, amount_a: U256, amount_b: U256) {
        let storage = self.get_mut();
        let sender = msg::source();
        let program_id = exec::program_id();

        // Validate that at least one of the provided amounts is non-zero
        if amount_a == U256::zero() && amount_b == U256::zero() {
            panic!("Amounts are zero");
        }

        // Check if the pool is being initialized
        if storage.reserve_a == U256::zero() && storage.reserve_b == U256::zero() {
            // Ensure both amounts are provided for initialization
            if amount_a == U256::zero() || amount_b == U256::zero() {
                panic!("Both tokens must be provided to initialize the pool");
            }

            // Initialize the pool with the first liquidity addition
            storage.reserve_a = amount_a;
            storage.reserve_b = amount_b;
            storage.total_liquidity = (amount_a * amount_b).integer_sqrt();

            // Assign initial liquidity tokens to the user
            let user_liquidity = storage
                .liquidity_providers
                .entry(sender)
                .or_insert(U256::zero());
            *user_liquidity += storage.total_liquidity;
        } else {
            // Ensure tokens are added proportionally to existing reserves
            let optimal_b = (amount_a * storage.reserve_b) / storage.reserve_a;
            let optimal_a = (amount_b * storage.reserve_a) / storage.reserve_b;

            if amount_b > optimal_b {
                panic!("Token B amount ({}) is too high for the provided Token A amount ({}). Optimal B: {}", amount_b, amount_a, optimal_b);
            }
            if amount_a > optimal_a {
                panic!("Token A amount ({}) is too high for the provided Token B amount ({}). Optimal A: {}", amount_a, amount_b, optimal_a);
            }

            // Calculate the liquidity tokens to mint
            let liquidity = U256::min(
                (amount_a * storage.total_liquidity) / storage.reserve_a,
                (amount_b * storage.total_liquidity) / storage.reserve_b,
            );

            // Update reserves and total liquidity
            storage.reserve_a += amount_a;
            storage.reserve_b += amount_b;
            storage.total_liquidity += liquidity;

            // Assign liquidity tokens to the user
            let user_liquidity = storage
                .liquidity_providers
                .entry(sender)
                .or_insert(U256::zero());
            *user_liquidity += liquidity;
        }

        let request_a = vft_io::TransferFrom::encode_call(sender, program_id, amount_a);
        msg::send_bytes_for_reply(storage.token_a, request_a, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        let request_b = vft_io::TransferFrom::encode_call(sender, program_id, amount_b);
        msg::send_bytes_for_reply(storage.token_b, request_b, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        self.notify_on(Event::AddedLiquidity {
            sender,
            amount_a,
            amount_b,
            liquidity: storage.total_liquidity,
        })
        .expect("Notification Error");
    }

    /// Removes liquidity from the pool.
    /// The amount of liquidity to remove is specified as `amount`.
    /// Proportionally calculates the amounts of token A and B to return to the user.
    /// Transfers the calculated token amounts back to the user.
    /// Updates the user's liquidity balance and the reserves of the pool.
    pub async fn remove_liquidity(&mut self, amount: U256) {
        let storage = self.get_mut();
        let sender = msg::source();
        let program_id = exec::program_id();

        // Fetch the user's liquidity and ensure they have enough to remove
        let user_liquidity = storage
            .liquidity_providers
            .get_mut(&sender)
            .expect("No liquidity");
        if *user_liquidity < amount {
            panic!("Insufficient liquidity");
        }

        let amount_a = (amount * storage.reserve_a) / storage.total_liquidity;
        let amount_b = (amount * storage.reserve_b) / storage.total_liquidity;

        storage.reserve_a -= amount_a;
        storage.reserve_b -= amount_b;
        storage.total_liquidity -= amount;
        *user_liquidity -= amount;

        // Transfer token A back to the user
        let request_a = vft_io::TransferFrom::encode_call(program_id, sender, amount_a);
        msg::send_bytes_for_reply(storage.token_a, request_a, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        // Transfer token B back to the user
        let request_b = vft_io::TransferFrom::encode_call(program_id, sender, amount_b);
        msg::send_bytes_for_reply(storage.token_b, request_b, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        self.notify_on(Event::RemovedLiquidity {
            sender,
            amount_a,
            amount_b,
            to: sender,
        })
        .expect("Notification Error");
    }

    /// Executes a token swap.
    /// The input token amount is specified as `in_amount`.
    /// Determines the output token based on `out_is_a`.
    /// Calculates the output amount using the reserves and updates them accordingly.
    /// Transfers the input token to the DEX and the output token to the user.
    pub async fn swap(&mut self, in_amount: U256, out_is_a: bool) {
        let storage = self.get_mut();
        let sender = msg::source();
        let program_id = exec::program_id();

        let (in_token, out_token, in_reserve, out_reserve) = if out_is_a {
            (
                storage.token_b,
                storage.token_a,
                &mut storage.reserve_b,
                &mut storage.reserve_a,
            )
        } else {
            (
                storage.token_a,
                storage.token_b,
                &mut storage.reserve_a,
                &mut storage.reserve_b,
            )
        };

        // Ensure the input amount is greater than zero
        if in_amount == U256::zero() {
            panic!("Input amount must be greater than zero");
        }

        // Ensure reserves are sufficient for the swap
        if *in_reserve == U256::zero() || *out_reserve == U256::zero() {
            panic!("Insufficient reserves for swap");
        }

        let out_amount = (in_amount * *out_reserve) / (*in_reserve + in_amount);

        // Ensure the reserves are sufficient to cover the output
        if out_amount > *out_reserve {
            panic!("Insufficient output reserves");
        }

        *in_reserve += in_amount;
        *out_reserve -= out_amount;

        // Transfer the input tokens to the contract
        let request_in = vft_io::TransferFrom::encode_call(sender, program_id, in_amount);
        msg::send_bytes_for_reply(in_token, request_in, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        // Transfer the output tokens to the user
        let request_out = vft_io::TransferFrom::encode_call(program_id, sender, out_amount);
        msg::send_bytes_for_reply(out_token, request_out, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        self.notify_on(Event::Swap {
            kind: if out_is_a {
                SwapKind::AForB
            } else {
                SwapKind::BForA
            },
            sender,
            in_amount,
            out_amount,
            to: sender,
        })
        .expect("Notification Error");
    }

    /// Synchronizes the contract's reserves with the actual token balances.
    /// Fetches the current balances of tokens A and B in the DEX contract.
    /// Updates the reserves based on the fetched balances.
    /// Notifies about the sync event.
    pub async fn sync(&mut self) {
        let storage = self.get_mut();

        // Fetch the current balance of token A in the contract
        let request = vft_io::BalanceOf::encode_call(exec::program_id());
        let bytes_reply_balances =
            msg::send_bytes_for_reply(storage.token_a, request.clone(), 0, 0)
                .expect("Error in async message to vft contract")
                .await
                .expect("Error getting answer from the vft contract");
        let balances_a: U256 = vft_io::BalanceOf::decode_reply(bytes_reply_balances).unwrap();

        // Fetch the current balance of token B in the contract
        let bytes_reply_balances = msg::send_bytes_for_reply(storage.token_b, request, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");
        let balances_b: U256 = vft_io::BalanceOf::decode_reply(bytes_reply_balances).unwrap();

        storage.reserve_a = balances_a;
        storage.reserve_b = balances_b;

        self.notify_on(Event::Sync {
            reserve_a: storage.reserve_a,
            reserve_b: storage.reserve_b,
        })
        .expect("Notification Error");
    }

    pub async fn kill(&mut self, inheritor: ActorId) {
        let storage = self.get();
        if storage.admin != msg::source() {
            panic!("Not Admin");
        }
        if let Some((id, _name)) = &storage.dns_info {
            let request = ["Dns".encode(), "DeleteMe".to_string().encode(), ().encode()].concat();

            msg::send_bytes_with_gas_for_reply(*id, request, 5_000_000_000, 0, 0)
                .expect("Error in sending message")
                .await
                .expect("Error in `AddNewProgram`");
        }

        self.notify_on(Event::Killed { inheritor })
            .expect("Notification Error");
        exec::exit(inheritor);
    }

    pub fn admin(&self) -> ActorId {
        self.get().admin
    }

    pub fn reserve_a(&self) -> U256 {
        self.get().reserve_a
    }

    pub fn reserve_b(&self) -> U256 {
        self.get().reserve_b
    }

    pub fn total_liquidity(&self) -> U256 {
        self.get().total_liquidity
    }

    pub fn liquidity_providers(&self) -> Vec<(ActorId, U256)> {
        self.get().liquidity_providers.clone().into_iter().collect()
    }

    pub fn token_a(&self) -> ActorId {
        self.get().token_a
    }

    pub fn token_b(&self) -> ActorId {
        self.get().token_b
    }
    pub fn dns_info(&self) -> Option<(ActorId, String)> {
        self.get().dns_info.clone()
    }
}

pub struct DexProgram(());

#[allow(clippy::new_without_default)]
#[sails_rs::program]
impl DexProgram {
    // Program's constructor
    pub async fn new(
        token_a: ActorId,
        token_b: ActorId,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        DexService::init(token_a, token_b, dns_id_and_name).await;
        Self(())
    }

    // Exposed service
    pub fn dex(&self) -> DexService {
        DexService::new()
    }
}
