#![no_std]

mod messages;

use gstd::{msg, prelude::*, prog::ProgramGenerator, ActorId};
use hashbrown::HashMap;
use messages::*;
use mt_logic_io::*;
use mt_storage_io::TokenId;
use primitive_types::H256;

const GAS_STORAGE_CREATION: u64 = 3_000_000_000;

#[derive(Default)]
struct MTLogic {
    admin: ActorId,
    mtoken_id: ActorId,
    transaction_status: HashMap<H256, TransactionStatus>,
    instructions: HashMap<H256, (Instruction, Instruction)>,
    storage_code_hash: H256,
    id_to_storage: HashMap<String, ActorId>,
    token_nonce: TokenId,
    token_uris: HashMap<TokenId, String>,
    token_total_supply: HashMap<TokenId, u128>,
    token_creators: HashMap<TokenId, ActorId>,
    nft_max_index: HashMap<TokenId, TokenId>,
    nft_owners: HashMap<TokenId, ActorId>,
}

impl MTLogic {
    async fn message(&mut self, transaction_hash: H256, msg_source: &ActorId, payload: &[u8]) {
        self.assert_main_contract();

        let action = Action::decode(&mut &payload[..]).expect("Can't decode `Action`");
        let transaction_status = self
            .transaction_status
            .get(&transaction_hash)
            .unwrap_or(&TransactionStatus::InProgress);

        match transaction_status {
            // The transaction has already been made but there wasn't enough gas for a message reply
            TransactionStatus::Success => reply_ok(),
            TransactionStatus::Failure => reply_err(),
            // The transaction took place for the first time
            // Or there was not enough gas to change the `TransactionStatus`
            TransactionStatus::InProgress => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::InProgress);

                match action {
                    Action::Transfer {
                        token_id,
                        sender,
                        recipient,
                        amount,
                    } => {
                        self.transfer(
                            transaction_hash,
                            token_id,
                            msg_source,
                            &sender,
                            &recipient,
                            amount,
                        )
                        .await
                    }
                    Action::Approve {
                        account,
                        is_approved,
                    } => {
                        self.approve(transaction_hash, msg_source, &account, is_approved)
                            .await
                    }
                    Action::Create {
                        initial_amount,
                        uri,
                        is_nft,
                    } => {
                        let _token_id = self
                            .create(transaction_hash, msg_source, initial_amount, uri, is_nft)
                            .await;
                    }
                    Action::MintBatchFT {
                        token_id,
                        to,
                        amounts,
                    } => {
                        self.mint_batch_ft(transaction_hash, token_id, msg_source, &to, amounts)
                            .await
                    }
                    Action::MintBatchNFT { token_id, to } => {
                        self.mint_batch_nft(transaction_hash, token_id, msg_source, &to)
                            .await
                    }
                    Action::BurnBatchFT {
                        token_id,
                        burn_from,
                        amounts,
                    } => {
                        self.burn_batch_ft(
                            transaction_hash,
                            token_id,
                            msg_source,
                            &burn_from,
                            amounts,
                        )
                        .await
                    }
                    Action::BurnNFT { token_id, from } => {
                        self.burn_nft(transaction_hash, token_id, msg_source, &from)
                            .await
                    }
                }
            }
        }
    }

    async fn transfer(
        &mut self,
        transaction_hash: H256,
        token_id: u128,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        if Self::is_nft(token_id) {
            // 1. Check that `msg_source` is eq to `sender` or approved
            if !self.is_approved(sender, msg_source).await {
                // Error, not approved
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return;
            }

            // 2. Check that `token_id` nft owner is `sender`
            if let Some(nft_owner) = self.nft_owners.get(&token_id) {
                if nft_owner != sender {
                    // Error, invalid owner
                    self.transaction_status
                        .insert(transaction_hash, TransactionStatus::Failure);
                    reply_err();
                    return;
                }
            } else {
                // Error, token not found
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return;
            }

            // 3. Set `token_id` nft owner to `recipient`
            self.nft_owners.insert(token_id, *recipient);

            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Success);
            reply_ok();
            return;
        }

        let sender_storage_id = self.get_or_create_storage_address(sender);
        let recipient_storage_id = self.get_or_create_storage_address(recipient);

        if recipient_storage_id == sender_storage_id {
            self.transfer_single_storage(
                transaction_hash,
                &sender_storage_id,
                token_id,
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
                    &sender_storage_id,
                    token_id,
                    msg_source,
                    sender,
                    amount,
                );
                let increase_instruction = create_increase_instruction(
                    transaction_hash,
                    &recipient_storage_id,
                    token_id,
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

    #[allow(clippy::too_many_arguments)]
    async fn transfer_single_storage(
        &mut self,
        transaction_hash: H256,
        storage_id: &ActorId,
        token_id: u128,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        let result = transfer(
            storage_id,
            transaction_hash,
            token_id,
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
        msg_source: &ActorId,
        account: &ActorId,
        is_approved: bool,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);
        let storage_id = self.get_or_create_storage_address(msg_source);

        let result = approve(
            &storage_id,
            transaction_hash,
            msg_source,
            account,
            is_approved,
        )
        .await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok();
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    async fn create(
        &mut self,
        transaction_hash: H256,
        msg_source: &ActorId,
        initial_amount: u128,
        uri: String,
        is_nft: bool,
    ) -> TokenId {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);

        let next_nonce = self.token_nonce.checked_add(1).expect("Math overflow!");

        // Store the type in the upper 64 bits
        // Before: 0 0 0 0 0 0 1
        // After:  0 0 0 1 0 0 0
        let mut token_type = next_nonce << (mem::size_of::<TokenId>() * 8 / 2);

        // Set a flag, if this is an NFT
        if is_nft {
            token_type |= NFT_BIT;
        }

        let token_id = token_type;
        self.token_nonce = next_nonce;

        self.token_uris.insert(token_id, uri);
        self.token_creators.insert(token_id, *msg_source);

        if !is_nft {
            self.token_total_supply.insert(token_id, initial_amount);

            let to_storage_id = self.get_or_create_storage_address(msg_source);
            let mut increase_instruction = create_increase_instruction(
                transaction_hash,
                &to_storage_id,
                token_id,
                msg_source,
                initial_amount,
            );

            if increase_instruction.start().await.is_err() {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return 0;
            }
        }

        self.transaction_status
            .insert(transaction_hash, TransactionStatus::Success);
        reply_ok();

        token_id
    }

    async fn mint_batch_ft(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: &ActorId,
        to: &Vec<ActorId>,
        amounts: Vec<u128>,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);

        if to.len() != amounts.len() || msg_source.is_zero() || !Self::is_ft(token_id) {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        // TODO: Check if `msg_source` can mint `token_id` token

        for (i, to) in to.iter().enumerate() {
            let amount = amounts[i];

            let to_storage_id = self.get_or_create_storage_address(to);
            let mut increase_instruction =
                create_increase_instruction(transaction_hash, &to_storage_id, token_id, to, amount);

            let token_total_supply = self
                .token_total_supply
                .get_mut(&token_id)
                .expect("Unable to locate token.");
            let new_token_total_supply = token_total_supply
                .checked_add(amount)
                .expect("Math overflow!");

            if increase_instruction.start().await.is_err() {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return;
            }

            *token_total_supply = new_token_total_supply;
        }

        self.transaction_status
            .insert(transaction_hash, TransactionStatus::Success);
        reply_ok();
    }

    async fn mint_batch_nft(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        _msg_source: &ActorId,
        to: &Vec<ActorId>,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);

        if !Self::is_nft(token_id) {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        // TODO: Check if `msg_source` can mint `token_id` token

        let index = self
            .nft_max_index
            .get(&token_id)
            .unwrap_or(&0)
            .checked_add(1)
            .expect("Math overflow!");
        self.nft_max_index.insert(
            token_id,
            (to.len() as u128)
                .checked_add(*self.nft_max_index.get(&token_id).unwrap_or(&0))
                .expect("Math overflow!"),
        );

        for (i, to) in to.iter().enumerate() {
            let id = token_id | (index + i as TokenId);

            self.nft_owners.insert(id, *to);
        }

        self.transaction_status
            .insert(transaction_hash, TransactionStatus::Success);
        reply_ok();
    }

    async fn burn_batch_ft(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: &ActorId,
        burn_from: &Vec<ActorId>,
        amounts: Vec<u128>,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);

        if burn_from.len() != amounts.len() || msg_source.is_zero() || !Self::is_ft(token_id) {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        for (i, from) in burn_from.iter().enumerate() {
            let amount = amounts[i];

            if !self.is_approved(from, msg_source).await {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return;
            }

            let from_storage_id = self.get_or_create_storage_address(from);
            let mut decrease_instruction = create_decrease_instruction(
                transaction_hash,
                &from_storage_id,
                token_id,
                msg_source,
                from,
                amount,
            );

            let token_total_supply = self
                .token_total_supply
                .get_mut(&token_id)
                .expect("Unable to locate token.");
            let new_token_total_supply = token_total_supply
                .checked_sub(amount)
                .expect("Math overflow!");

            if decrease_instruction.start().await.is_err() {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return;
            }

            *token_total_supply = new_token_total_supply;
        }

        self.transaction_status
            .insert(transaction_hash, TransactionStatus::Success);
        reply_ok();
    }

    async fn burn_nft(
        &mut self,
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: &ActorId,
        from: &ActorId,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);

        if !Self::is_nft(token_id) {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        // 1. Check that `msg_source` is eq to `from` or approved
        if !self.is_approved(from, msg_source).await {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        // 2. Check that `token_id` nft owner is `from`
        if let Some(nft_owner) = self.nft_owners.get(&token_id) {
            if nft_owner != from {
                // Error, invalid owner
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
                return;
            }
        } else {
            // Error, token not found
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        // 3. Remove `token_id` nft
        self.nft_owners.remove(&token_id);

        self.transaction_status
            .insert(transaction_hash, TransactionStatus::Success);
        reply_ok();
    }

    fn clear(&mut self, transaction_hash: H256) {
        self.transaction_status.remove(&transaction_hash);
    }

    fn update_storage_hash(&mut self, storage_code_hash: H256) {
        self.assert_admin();
        self.storage_code_hash = storage_code_hash;
    }

    fn get_or_create_storage_address(&mut self, address: &ActorId) -> ActorId {
        let encoded = hex::encode(address.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None.").to_string();
        if let Some(address) = self.id_to_storage.get(&id) {
            *address
        } else {
            let (_message_id, address) = ProgramGenerator::create_program_with_gas(
                self.storage_code_hash.into(),
                "",
                GAS_STORAGE_CREATION,
                0,
            )
            .expect("Error in creating Storage program.");
            self.id_to_storage.insert(id, address);
            address
        }
    }

    async fn get_balance(&self, token_id: TokenId, account: &ActorId) {
        if Self::is_nft(token_id) {
            let balance = match self.nft_owners.get(&token_id) {
                Some(owner) if owner == account => 1,
                Some(_) => 0,
                None => 0,
            };

            msg::reply(MTLogicEvent::Balance(balance), 0)
                .expect("Error in a reply `MTLogicEvent::Balance`.");
            return;
        }

        let encoded = hex::encode(account.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None.").to_string();

        if let Some(storage_id) = self.id_to_storage.get(&id) {
            let balance = get_balance(storage_id, token_id, account)
                .await
                .unwrap_or(0);

            msg::reply(MTLogicEvent::Balance(balance), 0)
                .expect("Error in a reply `MTLogicEvent::Balance`.");
        } else {
            msg::reply(MTLogicEvent::Balance(0), 0)
                .expect("Error in a reply `MTLogicEvent::Balance`.");
        }
    }

    async fn is_approved(&self, from: &ActorId, to: &ActorId) -> bool {
        let encoded = hex::encode(from.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None.").to_string();

        if let Some(storage_id) = self.id_to_storage.get(&id) {
            get_approval(storage_id, from, to).await.unwrap_or(false)
        } else {
            from == to
        }
    }

    async fn get_approval(&self, account: &ActorId, approval_target: &ActorId) {
        msg::reply(
            MTLogicEvent::Approval(self.is_approved(account, approval_target).await),
            0,
        )
        .expect("Error in a reply `MTLogicEvent::Approval`.");
    }

    fn is_ft(token_id: TokenId) -> bool {
        token_id & NFT_BIT == 0
    }

    fn is_nft(token_id: TokenId) -> bool {
        token_id & NFT_BIT == NFT_BIT
    }

    #[allow(unused)]
    fn get_nft_index(token_id: TokenId) -> TokenId {
        token_id & NFT_INDEX_MASK
    }

    #[allow(unused)]
    fn get_nft_base_type(token_id: TokenId) -> TokenId {
        token_id & NFT_TYPE_MASK
    }

    #[allow(unused)]
    fn is_nft_base_type(token_id: TokenId) -> bool {
        (token_id & NFT_BIT == NFT_BIT) && (token_id & NFT_INDEX_MASK == 0)
    }

    #[allow(unused)]
    fn is_nft_item(token_id: TokenId) -> bool {
        (token_id & NFT_BIT == NFT_BIT) && (token_id & NFT_INDEX_MASK != 0)
    }

    #[allow(unused)]
    fn get_token_uri(&self, token_id: TokenId) -> String {
        self.token_uris
            .get(&token_id)
            .expect("Unable to locate token.")
            .clone()
    }

    #[allow(unused)]
    fn get_token_creator(&self, token_id: TokenId) -> ActorId {
        *self
            .token_creators
            .get(&token_id)
            .expect("Unable to locate token.")
    }

    #[allow(unused)]
    fn get_token_total_supply(&self, token_id: TokenId) -> u128 {
        *self
            .token_total_supply
            .get(&token_id)
            .expect("Unable to locate token.")
    }

    fn assert_main_contract(&self) {
        assert_eq!(
            self.mtoken_id,
            msg::source(),
            "Only main multitoken contract can send that message"
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

static mut MT_LOGIC: Option<MTLogic> = None;

#[no_mangle]
extern "C" fn init() {
    let init_config: InitMTLogic = msg::load().expect("Unable to decode `InitMTLogic`");
    let mt_logic = MTLogic {
        admin: init_config.admin,
        storage_code_hash: init_config.storage_code_hash,
        mtoken_id: msg::source(),
        ..Default::default()
    };

    unsafe { MT_LOGIC = Some(mt_logic) };
}

#[gstd::async_main]
async fn main() {
    let action: MTLogicAction = msg::load().expect("Error in loading `MTLogicAction`");
    let logic: &mut MTLogic = unsafe { MT_LOGIC.get_or_insert(Default::default()) };

    match action {
        MTLogicAction::Message {
            transaction_hash,
            account,
            payload,
        } => logic.message(transaction_hash, &account, &payload).await,
        MTLogicAction::GetBalance { token_id, account } => {
            logic.get_balance(token_id, &account).await
        }
        MTLogicAction::GetApproval {
            account,
            approval_target,
        } => logic.get_approval(&account, &approval_target).await,
        MTLogicAction::UpdateStorageCodeHash(storage_code_hash) => {
            logic.update_storage_hash(storage_code_hash)
        }
        MTLogicAction::Clear(transaction_hash) => logic.clear(transaction_hash),
        _ => unimplemented!(),
    }
}

#[no_mangle]
extern "C" fn state() {
    let logic = unsafe { MT_LOGIC.as_ref().expect("Logic is not initialized.") };
    let logic_state = MTLogicState {
        admin: logic.admin,
        mtoken_id: logic.mtoken_id,
        transaction_status: logic
            .transaction_status
            .iter()
            .map(|(a, b)| (*a, *b))
            .collect(),
        instructions: logic
            .instructions
            .iter()
            .map(|(a, b)| (*a, b.clone()))
            .collect(),
        storage_code_hash: logic.storage_code_hash,
        id_to_storage: logic
            .id_to_storage
            .iter()
            .map(|(a, b)| (a.clone(), *b))
            .collect(),
        token_nonce: logic.token_nonce,
        token_uris: logic
            .token_uris
            .iter()
            .map(|(a, b)| (*a, b.clone()))
            .collect(),
        token_total_supply: logic
            .token_total_supply
            .iter()
            .map(|(a, b)| (*a, *b))
            .collect(),
        token_creators: logic.token_creators.iter().map(|(a, b)| (*a, *b)).collect(),
    };

    msg::reply(logic_state, 0).expect("Failed to share state.");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash.");
}

fn reply_err() {
    msg::reply(MTLogicEvent::Err, 0).expect("Error in sending a reply `MTLogicEvent::Err`");
}

fn reply_ok() {
    msg::reply(MTLogicEvent::Ok, 0).expect("Error in sending a reply `MTLogicEvent::Ok`");
}
