#![no_std]

use escrow_io::*;
use ft_io::{FTAction, FTEvent};
use gstd::{
    async_main, exec,
    msg::{self, CodecMessageFuture},
    prelude::*,
    ActorId,
};
use primitive_types::U256;

fn transfer_tokens(
    ft_program_id: ActorId,
    from: ActorId,
    to: ActorId,
    amount: u128,
) -> CodecMessageFuture<FTEvent> {
    msg::send_and_wait_for_reply(ft_program_id, FTAction::Transfer { from, to, amount }, 0).unwrap()
}

fn get(wallets: &mut BTreeMap<WalletId, Wallet>, wallet_id: WalletId) -> &mut Wallet {
    if let Some(wallet) = wallets.get_mut(&wallet_id) {
        wallet
    } else {
        panic!("Wallet with the {wallet_id} ID doesn't exist");
    }
}

#[derive(Default)]
struct Escrow {
    ft_program_id: ActorId,
    wallets: BTreeMap<WalletId, Wallet>,
    id_nonce: U256,
}

impl Escrow {
    /// Creates one escrow wallet and replies with its ID.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer or seller for this wallet.
    ///
    /// Arguments:
    /// * `buyer`: a buyer.
    /// * `seller`: a seller.
    /// * `amount`: an amount of tokens.
    fn create(&mut self, buyer: ActorId, seller: ActorId, amount: u128) {
        if msg::source() != buyer && msg::source() != seller {
            panic!("msg::source() must be a buyer or seller to create this escrow wallet");
        }

        let wallet_id = self.id_nonce;
        self.id_nonce = self.id_nonce.saturating_add(U256::one());

        self.wallets.insert(
            wallet_id,
            Wallet {
                buyer,
                seller,
                amount,
                state: WalletState::AwaitingDeposit,
            },
        );

        msg::reply(EscrowEvent::Created(wallet_id), 0).unwrap();
    }

    /// Makes a deposit from a buyer to an escrow wallet
    /// and changes a wallet state to `AwaitingConfirmation`.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer for this wallet.
    /// * Wallet must not be paid or closed.
    ///
    /// Arguments:
    /// * `wallet_id`: a wallet ID.
    async fn deposit(&mut self, wallet_id: WalletId) {
        let wallet = get(&mut self.wallets, wallet_id);

        if msg::source() != wallet.buyer {
            panic!("msg::source() must be a buyer for this wallet to make a deposit");
        }

        if wallet.state != WalletState::AwaitingDeposit {
            panic!("Paid or closed wallet can't take a deposit");
        }

        transfer_tokens(
            self.ft_program_id,
            wallet.buyer,
            exec::program_id(),
            wallet.amount,
        )
        .await
        .expect("Error when taking the deposit");

        wallet.state = WalletState::AwaitingConfirmation;

        msg::reply(
            EscrowEvent::Deposited {
                buyer: wallet.buyer,
                amount: wallet.amount,
            },
            0,
        )
        .unwrap();
    }

    /// Confirms a deal by transferring tokens from an escrow wallet
    /// to a seller and changing a wallet state to `Closed`.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer for this wallet.
    /// * Wallet must be paid and unclosed.
    ///
    /// Arguments:
    /// * `wallet_id`: a wallet ID.
    async fn confirm(&mut self, wallet_id: WalletId) {
        let wallet = get(&mut self.wallets, wallet_id);

        if msg::source() != wallet.buyer {
            panic!("msg::source() must a buyer for this wallet to confirm the deal");
        }

        if wallet.state != WalletState::AwaitingConfirmation {
            panic!("Deal can't be confirmed with the unpaid or closed wallet");
        }

        transfer_tokens(
            self.ft_program_id,
            exec::program_id(),
            wallet.seller,
            wallet.amount,
        )
        .await
        .expect("Error when transferring tokens to the seller");

        wallet.state = WalletState::Closed;

        msg::reply(
            EscrowEvent::Confirmed {
                amount: wallet.amount,
                seller: wallet.seller,
            },
            0,
        )
        .unwrap();
    }

    /// Refunds tokens from an escrow wallet to a buyer
    /// and changes a wallet state back to `AwaitingDeposit`
    /// (that is, a wallet can be reused).
    ///
    /// Requirements:
    /// * `msg::source()` must be a seller for this wallet.
    /// * Wallet must be paid and unclosed.
    ///
    /// Arguments:
    /// * `wallet_id`: a wallet ID.
    async fn refund(&mut self, wallet_id: WalletId) {
        let wallet = get(&mut self.wallets, wallet_id);

        if msg::source() != wallet.seller {
            panic!("msg::source() must be a seller for this wallet to refund");
        }

        if wallet.state != WalletState::AwaitingConfirmation {
            panic!("Unpaid or closed wallet can't be refunded");
        }

        transfer_tokens(
            self.ft_program_id,
            exec::program_id(),
            wallet.buyer,
            wallet.amount,
        )
        .await
        .expect("Error when refunding from the wallet");

        wallet.state = WalletState::AwaitingDeposit;

        msg::reply(
            EscrowEvent::Refunded {
                amount: wallet.amount,
                buyer: wallet.buyer,
            },
            0,
        )
        .unwrap();
    }

    /// Cancels a deal and closes an escrow wallet by changing its state to `Closed`.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer or seller for this wallet.
    /// * Wallet must not be paid or closed.
    ///
    /// Arguments:
    /// * `wallet_id`: a wallet ID.
    async fn cancel(&mut self, wallet_id: WalletId) {
        let wallet = get(&mut self.wallets, wallet_id);

        if msg::source() != wallet.buyer && msg::source() != wallet.seller {
            panic!("msg::source() must be a buyer or seller for this wallet to cancel the deal");
        }

        if wallet.state != WalletState::AwaitingDeposit {
            panic!("Deal can't be canceled with the paid or closed wallet");
        }

        wallet.state = WalletState::Closed;

        msg::reply(
            EscrowEvent::Cancelled {
                buyer: wallet.buyer,
                seller: wallet.seller,
                amount: wallet.amount,
            },
            0,
        )
        .unwrap();
    }
}

static mut ESCROW: Option<Escrow> = None;

#[no_mangle]
pub extern "C" fn init() {
    let config: InitEscrow = msg::load().expect("Unable to decode InitEscrow");
    let escrow = Escrow {
        ft_program_id: config.ft_program_id,
        ..Default::default()
    };
    unsafe {
        ESCROW = Some(escrow);
    }
}

#[async_main]
pub async fn main() {
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
pub extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: EscrowState = msg::load().expect("Unable to decode EscrowState");
    let escrow = unsafe { ESCROW.get_or_insert(Default::default()) };
    let encoded = match state {
        EscrowState::GetInfo(wallet_id) => {
            EscrowStateReply::Info(*get(&mut escrow.wallets, wallet_id)).encode()
        }
    };
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
