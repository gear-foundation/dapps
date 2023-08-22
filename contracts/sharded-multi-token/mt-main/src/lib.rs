#![no_std]

use gstd::{msg, prelude::*, prog::ProgramGenerator, ActorId};
use hashbrown::HashMap;
use mt_logic_io::{InitMTLogic, MTLogicAction, MTLogicEvent, TokenId};
use mt_main_io::*;
use primitive_types::H256;

#[derive(Default)]
struct MToken {
    admin: ActorId,
    mt_logic_id: ActorId,
    transactions: HashMap<H256, TransactionStatus>,
}

impl MToken {
    /// Accepts the payload message that will be sent to the logic token contract.
    ///
    /// Arguments:
    /// * `transaction_id`: the id of the transaction indicated by the actor that has sent that message;
    /// * `payload`: the message payload that will be sent to the logic token contract
    async fn message(&mut self, transaction_id: u64, payload: &[u8]) {
        // Get the transaction hash from `msg::source` and `transaction_id`
        // Tracking the trandaction ids is a responsibility of the account or programs that sent that transaction.
        let transaction_hash = get_hash(&msg::source(), transaction_id);
        let transaction = self.transactions.get(&transaction_hash);

        match transaction {
            None => {
                // If transaction took place for the first time we set its status to `InProgress`
                // and send message to the logic contract.
                self.transactions
                    .insert(transaction_hash, TransactionStatus::InProgress);
                self.send_message_then_reply(transaction_hash, payload)
                    .await;
            }
            // The case when there was not enough gas to process the result of the message to the logic contract.
            Some(transaction_status) => match transaction_status {
                TransactionStatus::InProgress => {
                    self.send_message_then_reply(transaction_hash, payload)
                        .await;
                }
                TransactionStatus::Success => {
                    reply_ok();
                }
                TransactionStatus::Failure => {
                    reply_err();
                }
            },
        }
    }

    async fn send_message_then_reply(&mut self, transaction_hash: H256, payload: &[u8]) {
        let result = self.send_message(transaction_hash, payload).await;
        match result {
            Ok(()) => {
                self.transactions
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok();
            }
            Err(()) => {
                self.transactions
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        };
    }

    async fn send_message(&self, transaction_hash: H256, payload: &[u8]) -> Result<(), ()> {
        let result = msg::send_for_reply_as::<MTLogicAction, MTLogicEvent>(
            self.mt_logic_id,
            MTLogicAction::Message {
                transaction_hash,
                account: msg::source(),
                payload: payload.to_vec(),
            },
            0,
        )
        .expect("Error in sending a message to the multitoken logic contract.")
        .await;

        match result {
            Ok(MTLogicEvent::Ok) => Ok(()),
            _ => Err(()),
        }
    }

    async fn get_balance(&self, token_id: TokenId, account: &ActorId) {
        let reply = msg::send_for_reply_as::<MTLogicAction, MTLogicEvent>(
            self.mt_logic_id,
            MTLogicAction::GetBalance {
                token_id,
                account: *account,
            },
            0,
        )
        .expect("Error in sending a message `MTLogicAction::GetBalance`.")
        .await
        .expect("Unable to decode `MTLogicEvent`.");

        if let MTLogicEvent::Balance(balance) = reply {
            msg::reply(MTokenEvent::Balance(balance), 0)
                .expect("Error in a reply `MTokenEvent::Balance`.");
        }
    }

    async fn get_approval(&self, account: &ActorId, approval_target: &ActorId) {
        let reply = msg::send_for_reply_as::<MTLogicAction, MTLogicEvent>(
            self.mt_logic_id,
            MTLogicAction::GetApproval {
                account: *account,
                approval_target: *approval_target,
            },
            0,
        )
        .expect("Error in sending a message `MTLogicAction::GetApproval`.")
        .await
        .expect("Unable to decode `MTLogicEvent`.");

        if let MTLogicEvent::Approval(approval) = reply {
            msg::reply(MTokenEvent::Approval(approval), 0)
                .expect("Error in a reply `MTokenEvent::Approval`.");
        }
    }

    fn update_logic_contract(&mut self, mt_logic_code_hash: H256, storage_code_hash: H256) {
        self.assert_admin();

        let (_message_id, mt_logic_id) = ProgramGenerator::create_program(
            mt_logic_code_hash.into(),
            InitMTLogic {
                admin: msg::source(),
                storage_code_hash,
            }
            .encode(),
            0,
        )
        .expect("Error in creating MToken Logic program");

        self.mt_logic_id = mt_logic_id;
    }

    fn assert_admin(&self) {
        assert!(
            msg::source() == self.admin,
            "Only admin can send that message"
        );
    }

    fn clear(&mut self, transaction_hash: H256) {
        self.transactions.remove(&transaction_hash);
    }
}

static mut MTOKEN: Option<MToken> = None;

#[no_mangle]
extern "C" fn init() {
    let init_config: InitMToken = msg::load().expect("Unable to decode `InitMToken`.");

    let (_message_id, mt_logic_id) = ProgramGenerator::create_program(
        init_config.mt_logic_code_hash.into(),
        InitMTLogic {
            admin: msg::source(),
            storage_code_hash: init_config.storage_code_hash,
        }
        .encode(),
        0,
    )
    .expect("Error in creating MToken Logic program");

    let mtoken = MToken {
        admin: msg::source(),
        mt_logic_id,
        ..Default::default()
    };

    unsafe { MTOKEN = Some(mtoken) };
}

#[gstd::async_main]
async fn main() {
    let action: MTokenAction = msg::load().expect("Unable to decode `MTokenAction`.");
    let mtoken: &mut MToken = unsafe { MTOKEN.get_or_insert(Default::default()) };

    match action {
        MTokenAction::Message {
            transaction_id,
            payload,
        } => {
            let payload_encoded = payload.encode();
            mtoken.message(transaction_id, &payload_encoded).await
        }
        MTokenAction::UpdateLogicContract {
            mt_logic_code_hash,
            storage_code_hash,
        } => mtoken.update_logic_contract(mt_logic_code_hash, storage_code_hash),
        MTokenAction::Clear(transaction_hash) => mtoken.clear(transaction_hash),
        MTokenAction::GetBalance { token_id, account } => {
            mtoken.get_balance(token_id, &account).await
        }
        MTokenAction::GetApproval {
            account,
            approval_target,
        } => mtoken.get_approval(&account, &approval_target).await,
        MTokenAction::MigrateStorageAddresses => {
            unimplemented!()
        }
    };
}

#[no_mangle]
extern "C" fn state() {
    let token = unsafe { MTOKEN.as_ref().expect("MToken is not initialized.") };
    let token_state = MTokenState {
        admin: token.admin,
        mt_logic_id: token.mt_logic_id,
        transactions: token.transactions.iter().map(|(a, b)| (*a, *b)).collect(),
    };

    msg::reply(token_state, 0).expect("Failed to share state.");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash.");
}

fn reply_ok() {
    msg::reply(MTokenEvent::Ok, 0).expect("Error in a reply `MTokenEvent::Ok`.");
}

fn reply_err() {
    msg::reply(MTokenEvent::Err, 0).expect("Error in a reply `MTokenEvent::Err`.");
}

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
}
