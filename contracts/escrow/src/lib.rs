#![no_std]

use escrow_io::*;
use ft_io::{FTAction, FTEvent};
use gstd::{async_main, exec, msg, prelude::*, ActorId};

async fn transfer_tokens(ft_program_id: ActorId, from: ActorId, to: ActorId, amount: u128) {
    msg::send_for_reply_as::<_, FTEvent>(ft_program_id, FTAction::Transfer { from, to, amount }, 0)
        .expect("Error during a sending FTAction::Transfer to a FT program")
        .await
        .expect("Unable to decode FTEvent");
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

    async fn deposit(&mut self, wallet_id: WalletId) {
        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer(wallet.buyer);
        assert_eq!(wallet.state, WalletState::AwaitingDeposit);

        transfer_tokens(
            self.ft_program_id,
            wallet.buyer,
            exec::program_id(),
            wallet.amount,
        )
        .await;
        wallet.state = WalletState::AwaitingConfirmation;

        reply(EscrowEvent::Deposited(wallet_id));
    }

    async fn confirm(&mut self, wallet_id: WalletId) {
        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer(wallet.buyer);
        assert_eq!(wallet.state, WalletState::AwaitingConfirmation);

        transfer_tokens(
            self.ft_program_id,
            exec::program_id(),
            wallet.seller,
            wallet.amount,
        )
        .await;
        wallet.state = WalletState::Closed;

        reply(EscrowEvent::Confirmed(wallet_id));
    }

    async fn refund(&mut self, wallet_id: WalletId) {
        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_seller(wallet.seller);
        assert_eq!(wallet.state, WalletState::AwaitingConfirmation);

        transfer_tokens(
            self.ft_program_id,
            exec::program_id(),
            wallet.buyer,
            wallet.amount,
        )
        .await;
        wallet.state = WalletState::AwaitingDeposit;

        reply(EscrowEvent::Refunded(wallet_id));
    }

    async fn cancel(&mut self, wallet_id: WalletId) {
        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer_or_seller(wallet.buyer, wallet.seller);
        assert_eq!(wallet.state, WalletState::AwaitingDeposit);

        wallet.state = WalletState::Closed;

        reply(EscrowEvent::Cancelled(wallet_id));
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
        EscrowAction::Deposit(wallet_id) => escrow.deposit(wallet_id).await,
        EscrowAction::Confirm(wallet_id) => escrow.confirm(wallet_id).await,
        EscrowAction::Refund(wallet_id) => escrow.refund(wallet_id).await,
        EscrowAction::Cancel(wallet_id) => escrow.cancel(wallet_id).await,
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
