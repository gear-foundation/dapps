#![no_std]

pub mod io;

use ft_main_io::*;
use gstd::{async_main, exec, msg, prelude::*, ActorId};

use crate::io::*;

/// Transfers `amount` tokens from `sender` account to `recipient` account.
/// Arguments:
/// * `transaction_id`: associated transaction id
/// * `from`: sender account
/// * `to`: recipient account
/// * `amount`: amount of tokens
async fn transfer_tokens(
    transaction_id: u64,
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount_tokens: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<_, FTokenEvent>(
        *token_address,
        FTokenAction::Message {
            transaction_id,
            payload: ft_logic_io::Action::Transfer {
                sender: *from,
                recipient: *to,
                amount: amount_tokens,
            }
            .encode(),
        },
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;

    match reply {
        Ok(FTokenEvent::Ok) => Ok(()),
        _ => Err(()),
    }
}

fn get_mut_wallet(wallets: &mut BTreeMap<WalletId, Wallet>, wallet_id: WalletId) -> &mut Wallet {
    wallets
        .get_mut(&wallet_id)
        .unwrap_or_else(|| panic_wallet_not_exist(wallet_id))
}

fn reply(escrow_event: EscrowEvent) {
    msg::reply(escrow_event, 0).expect("Error during a replying with EscrowEvent");
}

fn check_buyer_or_seller(buyer: ActorId, seller: ActorId) {
    if msg::source() != buyer && msg::source() != seller {
        panic!("msg::source() must be a buyer or seller");
    }
}

fn check_buyer(buyer: ActorId) {
    if msg::source() != buyer {
        panic!("msg::source() must be a buyer");
    }
}

fn check_seller(seller: ActorId) {
    if msg::source() != seller {
        panic!("msg::source() must be a seller");
    }
}

fn panic_wallet_not_exist(wallet_id: WalletId) -> ! {
    panic!("Wallet with the {wallet_id} ID doesn't exist");
}

#[derive(Default)]
struct Escrow {
    ft_program_id: ActorId,
    wallets: BTreeMap<WalletId, Wallet>,
    id_nonce: WalletId,
    transaction_id: u64,
    transactions: BTreeMap<u64, Option<EscrowAction>>,
}

impl Escrow {
    fn create(&mut self, buyer: ActorId, seller: ActorId, amount: u128) {
        if buyer == ActorId::zero() && seller == ActorId::zero() {
            panic!("A buyer or seller can't have the zero address")
        }
        check_buyer_or_seller(buyer, seller);

        let wallet_id = self.id_nonce;
        self.id_nonce = self.id_nonce.saturating_add(WalletId::one());

        if self.wallets.contains_key(&wallet_id) {
            panic!("Wallet with the {wallet_id} already exists");
        }
        self.wallets.insert(
            wallet_id,
            Wallet {
                buyer,
                seller,
                amount,
                state: WalletState::AwaitingDeposit,
            },
        );

        reply(EscrowEvent::Created(wallet_id));
    }

    async fn deposit(&mut self, transaction_id: Option<u64>, wallet_id: WalletId) {
        let current_transaction_id = self.get_transaction_id(transaction_id);

        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer(wallet.buyer);
        assert_eq!(wallet.state, WalletState::AwaitingDeposit);

        if transfer_tokens(
            current_transaction_id,
            &self.ft_program_id,
            &wallet.buyer,
            &exec::program_id(),
            wallet.amount,
        )
        .await
        .is_err()
        {
            self.transactions.remove(&current_transaction_id);
            reply(EscrowEvent::TransactionFailed);
            return;
        }

        wallet.state = WalletState::AwaitingConfirmation;

        self.transactions.remove(&current_transaction_id);

        reply(EscrowEvent::Deposited(current_transaction_id, wallet_id));
    }

    async fn confirm(&mut self, transaction_id: Option<u64>, wallet_id: WalletId) {
        let current_transaction_id = self.get_transaction_id(transaction_id);

        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer(wallet.buyer);
        assert_eq!(wallet.state, WalletState::AwaitingConfirmation);

        if transfer_tokens(
            current_transaction_id,
            &self.ft_program_id,
            &exec::program_id(),
            &wallet.seller,
            wallet.amount,
        )
        .await
        .is_ok()
        {
            wallet.state = WalletState::Closed;

            self.transactions.remove(&current_transaction_id);

            reply(EscrowEvent::Confirmed(current_transaction_id, wallet_id));
        } else {
            reply(EscrowEvent::TransactionFailed);
        }
    }

    async fn refund(&mut self, transaction_id: Option<u64>, wallet_id: WalletId) {
        let current_transaction_id = self.get_transaction_id(transaction_id);

        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_seller(wallet.seller);
        assert_eq!(wallet.state, WalletState::AwaitingConfirmation);

        if transfer_tokens(
            current_transaction_id,
            &self.ft_program_id,
            &exec::program_id(),
            &wallet.buyer,
            wallet.amount,
        )
        .await
        .is_ok()
        {
            wallet.state = WalletState::AwaitingDeposit;

            self.transactions.remove(&current_transaction_id);

            reply(EscrowEvent::Refunded(current_transaction_id, wallet_id));
        } else {
            reply(EscrowEvent::TransactionFailed);
        }
    }

    async fn cancel(&mut self, wallet_id: WalletId) {
        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer_or_seller(wallet.buyer, wallet.seller);
        assert_eq!(wallet.state, WalletState::AwaitingDeposit);

        wallet.state = WalletState::Closed;

        reply(EscrowEvent::Cancelled(wallet_id));
    }

    /// Continues cached transaction by `transaction_id`.
    ///
    /// Execution makes sense if, when returning from an async message,
    /// the gas ran out and the state has changed.
    async fn continue_transaction(&mut self, transaction_id: u64) {
        let transactions = self.transactions.clone();
        let payload = &transactions
            .get(&transaction_id)
            .expect("Transaction does not exist");
        if let Some(action) = payload {
            match action {
                EscrowAction::Deposit(wallet_id) => {
                    self.deposit(Some(transaction_id), *wallet_id).await
                }
                EscrowAction::Confirm(wallet_id) => {
                    self.confirm(Some(transaction_id), *wallet_id).await
                }
                EscrowAction::Refund(wallet_id) => {
                    self.refund(Some(transaction_id), *wallet_id).await
                }
                _ => unreachable!(),
            }
        } else {
            msg::reply(EscrowEvent::TransactionProcessed, 0)
                .expect("Error in a reply `EscrowEvent::TransactionProcessed`");
        }
    }

    fn get_transaction_id(&mut self, transaction_id: Option<u64>) -> u64 {
        match transaction_id {
            Some(transaction_id) => transaction_id,
            None => {
                let transaction_id = self.transaction_id;
                self.transaction_id = self.transaction_id.wrapping_add(1);
                transaction_id
            }
        }
    }
}

static mut ESCROW: Option<Escrow> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitEscrow = msg::load().expect("Unable to decode InitEscrow");

    if config.ft_program_id == ActorId::zero() {
        panic!("FT program address can't be 0");
    }

    let escrow = Escrow {
        ft_program_id: config.ft_program_id,
        ..Default::default()
    };
    unsafe {
        ESCROW = Some(escrow);
    }
}

#[async_main]
async fn main() {
    let action: EscrowAction = msg::load().expect("Unable to decode EscrowAction");
    let escrow = unsafe { ESCROW.get_or_insert(Default::default()) };
    match action {
        EscrowAction::Create {
            buyer,
            seller,
            amount,
        } => escrow.create(buyer, seller, amount),
        EscrowAction::Deposit(wallet_id) => {
            escrow
                .transactions
                .insert(escrow.transaction_id, Some(action));
            escrow.deposit(None, wallet_id).await
        }
        EscrowAction::Confirm(wallet_id) => {
            escrow
                .transactions
                .insert(escrow.transaction_id, Some(action));
            escrow.confirm(None, wallet_id).await
        }
        EscrowAction::Refund(wallet_id) => {
            escrow
                .transactions
                .insert(escrow.transaction_id, Some(action));
            escrow.refund(None, wallet_id).await
        }
        EscrowAction::Cancel(wallet_id) => escrow.cancel(wallet_id).await,
        EscrowAction::Continue(transaction_id) => escrow.continue_transaction(transaction_id).await,
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: EscrowState = msg::load().expect("Unable to decode EscrowState");
    let escrow = unsafe { ESCROW.get_or_insert(Default::default()) };
    let encoded = match state {
        EscrowState::Info(wallet_id) => EscrowStateReply::Info(
            *escrow
                .wallets
                .get(&wallet_id)
                .unwrap_or_else(|| panic_wallet_not_exist(wallet_id)),
        ),
        EscrowState::CreatedWallets => EscrowStateReply::CreatedWallets(
            escrow
                .wallets
                .iter()
                .map(|(wallet_id, wallet)| (*wallet_id, *wallet))
                .collect(),
        ),
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "Escrow",
    init:
        input: InitEscrow,
    handle:
        input: EscrowAction,
        output: EscrowEvent,
    state:
        input: EscrowState,
        output: EscrowStateReply,
}
