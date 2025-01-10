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

const MINIMUM_LIQUIDITY: u128 = 1_000;

#[derive(Clone, Debug)]
pub struct Storage {
    pub admin: ActorId,
    pub reserve_a: U256,
    pub reserve_b: U256,
    pub total_liquidity: U256,
    pub liquidity_providers: HashMap<ActorId, U256>,
    pub token_a: ActorId,
    pub token_b: ActorId,
    pub k_last: U256, // reserve_a * reserve_b, updated after liquidity changes
    pub dns_info: Option<(ActorId, String)>,
    pub liquidity_action_gas: u64,
    pub swap_status: SwapStatus,
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
        liquidity_action_gas: u64,
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
                k_last: U256::zero(),
                dns_info: dns_id_and_name.clone(),
                liquidity_action_gas,
                swap_status: SwapStatus::Ready,
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
    /// Updates the user's liquidity balance and reserves.
    pub async fn add_liquidity(&mut self, amount_a: U256, amount_b: U256) {
        let storage = self.get_mut();
        if exec::gas_available() < storage.liquidity_action_gas {
            panic!("Not enough gas; requires a least: {:?}", storage.liquidity_action_gas);
        }

        let sender = msg::source();
        let program_id = exec::program_id();

        if amount_a.is_zero() || amount_b.is_zero() {
            panic!("Amounts must be greater than zero");
        }

        check_approve(&storage.token_a, &sender, &program_id, amount_a).await;
        check_approve(&storage.token_b, &sender, &program_id, amount_b).await;
    
        if storage.reserve_a.is_zero() && storage.reserve_b.is_zero() {
            // Initial liquidity
            let liquidity = (amount_a * amount_b).integer_sqrt();
            if liquidity < MINIMUM_LIQUIDITY.into() {
                panic!("Liquidity is low");
            }
            let liquidity_to_mint = liquidity - MINIMUM_LIQUIDITY;
            storage.reserve_a = amount_a;
            storage.reserve_b = amount_b;
            storage.total_liquidity = liquidity_to_mint;
            storage.liquidity_providers.insert(sender, liquidity_to_mint);
        } else {
            // Ensure tokens are added in correct proportions
            let expected_b = (amount_a * storage.reserve_b) / storage.reserve_a;
            let expected_a = (amount_b * storage.reserve_a) / storage.reserve_b;
    
            if amount_b != expected_b && amount_a != expected_a {
                panic!("Tokens must be provided in correct proportions");
            }
    
            let liquidity = U256::min(
                (amount_a * storage.total_liquidity) / storage.reserve_a,
                (amount_b * storage.total_liquidity) / storage.reserve_b,
            );
    
            if liquidity.is_zero() {
                panic!("Insufficient liquidity minted");
            }
    
            storage.reserve_a += amount_a;
            storage.reserve_b += amount_b;
            storage.total_liquidity += liquidity;
    
            let user_liquidity = storage
                .liquidity_providers
                .entry(sender)
                .or_insert(U256::zero());
            *user_liquidity += liquidity;
        }
    
        storage.k_last = storage.reserve_a * storage.reserve_b;

        // Transfer tokens to contract
        let request_a = vft_io::TransferFrom::encode_call(sender, program_id, amount_a);
        msg::send_bytes_with_gas_for_reply(storage.token_a, request_a, 5_000_000_000, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");
    
        let request_b = vft_io::TransferFrom::encode_call(sender, program_id, amount_b);
        msg::send_bytes_with_gas_for_reply(storage.token_b, request_b, 5_000_000_000, 0, 0)
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
    /// Transfers proportional token amounts back to the user.
    pub async fn remove_liquidity(&mut self, amount: U256) {
        let storage = self.get_mut();

        if exec::gas_available() < storage.liquidity_action_gas {
            panic!("Not enough gas; requires a least: {:?}", storage.liquidity_action_gas);
        }
        let sender = msg::source();
        let program_id = exec::program_id();

        let user_liquidity = storage
            .liquidity_providers
            .get_mut(&sender)
            .expect("No liquidity");

        if *user_liquidity < amount {
            panic!("Insufficient liquidity");
        }

        let amount_a = (amount * storage.reserve_a) / storage.total_liquidity;
        let amount_b = (amount * storage.reserve_b) / storage.total_liquidity;

        if storage.reserve_a < amount_a || storage.reserve_b < amount_b {
            panic!("Insufficient contract balance for token transfer");
        }

        // Transfer tokens back to the user
        let request_a = vft_io::TransferFrom::encode_call(program_id, sender, amount_a);
        msg::send_bytes_with_gas_for_reply(storage.token_a, request_a, 5_000_000_000, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        let request_b = vft_io::TransferFrom::encode_call(program_id, sender, amount_b);
        msg::send_bytes_with_gas_for_reply(storage.token_b, request_b, 5_000_000_000, 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");

        storage.reserve_a -= amount_a;
        storage.reserve_b -= amount_b;
        storage.total_liquidity -= amount;
        *user_liquidity -= amount;

        storage.k_last = storage.reserve_a * storage.reserve_b;

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
        if storage.swap_status != SwapStatus::Ready {
            panic!("Swap status is incorrect");
        }
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

        check_approve(&in_token, &sender, &program_id, in_amount).await;
        storage.swap_status = SwapStatus::Paused;

        // Transfer the input tokens to the contract
        let request_in = vft_io::TransferFrom::encode_call(sender, program_id, in_amount);
        msg::send_bytes_with_gas_for_reply(in_token, request_in, 5_000_000_000, 0, 5_000_000_000)
            .expect("Error in async message to vft contract")
            .up_to(Some(5))
            .expect("Reply timeout")
            .handle_reply(|| {
                let reply_bytes = msg::load_bytes().expect("Unable to load bytes");
                let result = vft_io::TransferFrom::decode_reply(reply_bytes);
                if result.is_err() {
                    let storage = unsafe { STORAGE.as_mut().expect("Dex is not initialized") };
                    storage.swap_status = SwapStatus::Ready;
                }
            })
            .expect("Reply hook error")
            .await
            .expect("Error getting answer from the vft contract");
        
        // Transfer the output tokens to the user
        let request_out = vft_io::TransferFrom::encode_call(program_id, sender, out_amount);
        msg::send_bytes_with_gas_for_reply(out_token, request_out, 5_000_000_000, 0, 5_000_000_000)
            .expect("Error in async message to vft contract")
            .up_to(Some(5))
            .expect("Reply timeout")
            .handle_reply(move || handle_reply_hook_for_output_tokens(out_token, sender, in_amount, out_amount, out_is_a))
            .expect("Reply hook error")
            .await
            .expect("Error getting answer from the vft contract");

        *in_reserve += in_amount;
        *out_reserve -= out_amount;
        storage.swap_status = SwapStatus::Ready;

        self.notify_on(Event::Swap {
            kind: if out_is_a {
                SwapKind::AForB
            } else {
                SwapKind::BForA
            },
            sender,
            in_amount,
            out_amount,
        })
        .expect("Notification Error");
    }

    pub async fn continue_swap(&mut self) {
        let storage = self.get_mut();

        let (to, in_amount, out_amount, out_is_a) = match storage.swap_status {
            SwapStatus::TokenTransferError { out_token, to, in_amount, out_amount, out_is_a } => {
                // Transfer the output tokens to the user
                let request_out = vft_io::TransferFrom::encode_call(exec::program_id(), to, out_amount);
                msg::send_bytes_with_gas_for_reply(out_token, request_out, 5_000_000_000, 0, 0)
                    .expect("Error in async message to vft contract")
                    .await
                    .expect("Error getting answer from the vft contract");
                (to, in_amount, out_amount, out_is_a)
            }
            SwapStatus::TokenTransferOk { to, in_amount, out_amount, out_is_a } => (to, in_amount, out_amount, out_is_a),
            _ => panic!("Swap status is incorrect")

        };
        if out_is_a {
            storage.reserve_b += in_amount;
            storage.reserve_a -= out_amount;
        } else {
            storage.reserve_a += in_amount;
            storage.reserve_b -= out_amount;
        };

        storage.swap_status = SwapStatus::Ready;

        self.notify_on(Event::Swap {
            kind: if out_is_a {
                SwapKind::AForB
            } else {
                SwapKind::BForA
            },
            sender: to,
            in_amount,
            out_amount,
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
    pub fn swap_status(&self) -> SwapStatus {
        self.get().swap_status
    }
    pub fn liquidity_action_gas(&self) -> u64 {
        self.get().liquidity_action_gas
    }
    
}

async fn check_approve(program_id: &ActorId, owner: &ActorId, spender: &ActorId, expected_allowance: U256) {
    let request = vft_io::Allowance::encode_call(*owner, *spender);
    let bytes_reply =
        msg::send_bytes_for_reply(*program_id, request.clone(), 0, 0)
            .expect("Error in async message to vft contract")
            .await
            .expect("Error getting answer from the vft contract");
    let allowance = vft_io::Allowance::decode_reply(bytes_reply).unwrap();
    if allowance < expected_allowance {
        panic!("The number of approved tokens is lower than expected")
    }
}

fn handle_reply_hook_for_output_tokens(out_token: ActorId, to: ActorId, in_amount: U256, out_amount: U256, out_is_a: bool) {
    let reply_bytes = msg::load_bytes().expect("Unable to load bytes");
    let result = vft_io::TransferFrom::decode_reply(reply_bytes);
    let storage = unsafe { STORAGE.as_mut().expect("Dex is not initialized") };
    if result.is_err() {
        storage.swap_status = SwapStatus::TokenTransferError { out_token, to, in_amount, out_amount, out_is_a };
    } else {
        storage.swap_status = SwapStatus::TokenTransferOk { to, in_amount, out_amount, out_is_a };
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
        liquidity_action_gas: u64,
        dns_id_and_name: Option<(ActorId, String)>,
    ) -> Self {
        DexService::init(token_a, token_b, liquidity_action_gas, dns_id_and_name).await;
        Self(())
    }

    // Exposed service
    pub fn dex(&self) -> DexService {
        DexService::new()
    }
}
