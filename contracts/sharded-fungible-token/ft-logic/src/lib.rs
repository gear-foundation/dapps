#![no_std]
use ft_logic_io::instruction::*;
use ft_logic_io::*;
use ft_main_io::LogicAction;
use gstd::{exec, msg, prelude::*, prog::ProgramGenerator, ActorId};

mod messages;
use hashbrown::HashMap;
use messages::*;
use primitive_types::{H256, H512};

const GAS_STORAGE_CREATION: u64 = 3_000_000_000;
const DELAY: u32 = 600_000;

#[derive(Default)]
struct FTLogic {
    admin: ActorId,
    ftoken_id: ActorId,
    transaction_status: HashMap<H256, TransactionStatus>,
    instructions: HashMap<H256, (Instruction, Instruction)>,
    storage_code_hash: H256,
    id_to_storage: HashMap<String, ActorId>,
}

static mut FT_LOGIC: Option<FTLogic> = None;

impl FTLogic {
    /// The message received from the main contract.
    ///
    /// Arguments:
    /// * `transaction_hash`: the hash associated with that transaction;
    /// * `account`: the account that sent the message to the main contract;
    /// * `action`: the message payload.
    async fn message(&mut self, transaction_hash: H256, account: &ActorId, payload: &[u8]) {
        self.assert_main_contract();
        let action = LogicAction::decode(&mut &payload[..]).expect("Can't decode `Action`");

        let transaction_status = self
            .transaction_status
            .get(&transaction_hash)
            .unwrap_or(&TransactionStatus::InProgress);

        match transaction_status {
            // The transaction has already been made but there wasn't enough gas for a message reply.
            TransactionStatus::Success => reply_ok(),
            TransactionStatus::Failure => reply_err(),
            // The transaction took place for the first time
            // Or there was not enough gas to change the `TransactionStatus`.
            TransactionStatus::InProgress => {
                send_delayed_clear(transaction_hash);
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::InProgress);
                match action {
                    LogicAction::Mint { recipient, amount } => {
                        self.mint(transaction_hash, &recipient, amount).await;
                    }
                    LogicAction::Burn { sender, amount } => {
                        self.burn(transaction_hash, account, &sender, amount).await;
                    }
                    LogicAction::Transfer {
                        sender,
                        recipient,
                        amount,
                    } => {
                        self.transfer(transaction_hash, account, &sender, &recipient, amount)
                            .await;
                    }
                    LogicAction::Approve {
                        approved_account,
                        amount,
                    } => {
                        self.approve(transaction_hash, account, &approved_account, amount)
                            .await;
                    }
                    LogicAction::Permit {
                        owner_account,
                        approved_account,
                        amount,
                        permit_id,
                        sign,
                    } => {
                        let payload = PermitUnsigned {
                            owner_account,
                            approved_account,
                            amount,
                            permit_id,
                        };
                        self.permit(
                            transaction_hash,
                            &owner_account,
                            &approved_account,
                            amount,
                            &sign,
                            &payload,
                        )
                        .await;
                    }
                }
            }
        }
    }

    async fn mint(&mut self, transaction_hash: H256, recipient: &ActorId, amount: u128) {
        let recipient_storage = self.get_storage_address(recipient);

        let result =
            increase_balance(transaction_hash, &recipient_storage, recipient, amount).await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok()
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    async fn burn(
        &mut self,
        transaction_hash: H256,
        account: &ActorId,
        sender: &ActorId,
        amount: u128,
    ) {
        let sender_storage = self.get_storage_address(sender);

        let result =
            decrease_balance(transaction_hash, &sender_storage, account, sender, amount).await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok()
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    async fn transfer(
        &mut self,
        transaction_hash: H256,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        let sender_storage = self.get_storage_address(sender);
        let recipient_storage = self.get_storage_address(recipient);

        if recipient_storage == sender_storage {
            self.transfer_single_storage(
                transaction_hash,
                &sender_storage,
                msg_source,
                sender,
                recipient,
                amount,
            )
            .await;
            return;
        }
        let (decrease_instruction, increase_instruction) = self
            .instructions
            .entry(transaction_hash)
            .or_insert_with(|| {
                let decrease_instruction = create_decrease_instruction(
                    transaction_hash,
                    msg_source,
                    &sender_storage,
                    sender,
                    amount,
                );
                let increase_instruction = create_increase_instruction(
                    transaction_hash,
                    &recipient_storage,
                    recipient,
                    amount,
                );
                (decrease_instruction, increase_instruction)
            });

        if decrease_instruction.start().await.is_err() {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        match increase_instruction.start().await {
            Err(_) => {
                if decrease_instruction.abort().await.is_ok() {
                    self.transaction_status
                        .insert(transaction_hash, TransactionStatus::Failure);
                    reply_err();
                }
            }
            Ok(_) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok();
            }
        }
    }

    async fn transfer_single_storage(
        &mut self,
        transaction_hash: H256,
        storage_id: &ActorId,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        let result = transfer(
            transaction_hash,
            storage_id,
            msg_source,
            sender,
            recipient,
            amount,
        )
        .await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok()
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    async fn approve(
        &mut self,
        transaction_hash: H256,
        account: &ActorId,
        approved_account: &ActorId,
        amount: u128,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);
        let account_storage = self.get_storage_address(account);

        let result = approve(
            transaction_hash,
            &account_storage,
            account,
            approved_account,
            amount,
        )
        .await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok()
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    fn check_signature(message: &PermitUnsigned, owner: &ActorId, sign: &H512) -> bool {
        let message_u8 = message.encode();
        light_sr25519::verify(sign.as_bytes(), message_u8, owner).is_ok()
    }

    async fn permit(
        &mut self,
        transaction_hash: H256,
        owner: &ActorId,
        spender: &ActorId,
        amount: u128,
        owner_sign: &H512,
        message: &PermitUnsigned,
    ) {
        if !FTLogic::check_signature(message, owner, owner_sign) {
            reply_err();
            return;
        }

        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);

        if !self
            .check_and_increment_permit_id(transaction_hash, owner, &message.permit_id)
            .await
        {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        let account_storage = self.get_storage_address(owner);
        let result = approve(transaction_hash, &account_storage, owner, spender, amount).await;
        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok()
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    fn update_storage_hash(&mut self, storage_code_hash: H256) {
        self.assert_admin();
        self.storage_code_hash = storage_code_hash;
    }

    fn get_storage_address(&mut self, address: &ActorId) -> ActorId {
        let encoded = hex::encode(address.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();
        if let Some(address) = self.id_to_storage.get(&id) {
            *address
        } else {
            let (_message_id, address) = ProgramGenerator::create_program_with_gas(
                self.storage_code_hash.into(),
                "",
                GAS_STORAGE_CREATION,
                0,
            )
            .expect("Error in creating Storage program");
            self.id_to_storage.insert(id, address);
            address
        }
    }

    async fn get_permit_id(&self, account: &ActorId) {
        let encoded = hex::encode(account.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();
        if let Some(address) = self.id_to_storage.get(&id) {
            let permit_id = get_permit_id(address, account).await;
            msg::reply(FTLogicEvent::PermitId(permit_id), 0)
                .expect("Error in a reply `FTLogicEvent::PermitId`");
        } else {
            msg::reply(FTLogicEvent::PermitId(0), 0)
                .expect("Error in a reply `FTLogicEvent::PermitId`");
        }
    }

    async fn check_and_increment_permit_id(
        &self,
        transaction_hash: H256,
        account: &ActorId,
        expected_id: &u128,
    ) -> bool {
        let encoded = hex::encode(account.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();
        if let Some(address) = self.id_to_storage.get(&id) {
            return check_and_increment_permit_id(address, transaction_hash, account, *expected_id)
                .await;
        }
        false
    }

    async fn get_balance(&self, account: &ActorId) {
        let encoded = hex::encode(account.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();
        if let Some(address) = self.id_to_storage.get(&id) {
            let balance = get_balance(address, account).await;
            msg::reply(FTLogicEvent::Balance(balance), 0)
                .expect("Error in a reply `FTLogicEvent::Balance`");
        } else {
            msg::reply(FTLogicEvent::Balance(0), 0)
                .expect("Error in a reply `FTLogicEvent::Balance`");
        }
    }

    fn clear(&mut self, transaction_hash: H256) {
        self.transaction_status.remove(&transaction_hash);
    }

    fn assert_main_contract(&self) {
        assert_eq!(
            self.ftoken_id,
            msg::source(),
            "Only main fungible token contract can send that message"
        );
    }

    fn assert_admin(&self) {
        assert_eq!(
            self.admin,
            msg::source(),
            "Only admin can send that message"
        );
    }
}

#[gstd::async_main]
async fn main() {
    let action: FTLogicAction = msg::load().expect("Error in loading `StorageAction`");
    let logic: &mut FTLogic = unsafe { FT_LOGIC.get_or_insert(Default::default()) };
    match action {
        FTLogicAction::Message {
            transaction_hash,
            account,
            payload,
        } => logic.message(transaction_hash, &account, &payload).await,
        FTLogicAction::UpdateStorageCodeHash(storage_code_hash) => {
            logic.update_storage_hash(storage_code_hash)
        }
        FTLogicAction::Clear(transaction_hash) => logic.clear(transaction_hash),
        FTLogicAction::GetBalance(account) => logic.get_balance(&account).await,
        FTLogicAction::GetPermitId(account) => logic.get_permit_id(&account).await,
        _ => {}
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_config: InitFTLogic = msg::load().expect("Unable to decode `InitFTLogic`");
    let ft_logic = FTLogic {
        admin: init_config.admin,
        storage_code_hash: init_config.storage_code_hash,
        ftoken_id: msg::source(),
        ..Default::default()
    };
    FT_LOGIC = Some(ft_logic);
}

fn reply_err() {
    msg::reply(FTLogicEvent::Err, 0).expect("Error in sending a reply `FTLogicEvent::Err`");
}

fn reply_ok() {
    msg::reply(FTLogicEvent::Ok, 0).expect("Error in sending a reply `FTLogicEvent::Ok`");
}

fn send_delayed_clear(transaction_hash: H256) {
    msg::send_delayed(
        exec::program_id(),
        FTLogicAction::Clear(transaction_hash),
        0,
        DELAY,
    )
    .expect("Error in sending a delayled message `FTStorageAction::Clear`");
}

#[no_mangle]
extern "C" fn state() {
    let logic = unsafe { FT_LOGIC.as_ref().expect("FTLogic is not initialized") };
    let logic_state = FTLogicState {
        admin: logic.admin,
        ftoken_id: logic.ftoken_id,
        transaction_status: logic
            .transaction_status
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
        instructions: logic
            .instructions
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
        storage_code_hash: logic.storage_code_hash,
        id_to_storage: logic
            .id_to_storage
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect(),
    };
    msg::reply(logic_state, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
