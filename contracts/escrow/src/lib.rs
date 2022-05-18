#![no_std]

use escrow_io::*;
use ft_io::*;
use gstd::{
    async_main, exec,
    msg::{self, CodecMessageFuture},
    prelude::*,
    ActorId,
};

#[derive(PartialEq)]
enum State {
    AwaitingDeposit,
    AwaitingConfirmation,
    Completed,
}

fn transfer_tokens(
    ft_program_id: ActorId,
    from: ActorId,
    to: ActorId,
    amount: u128,
) -> CodecMessageFuture<FTEvent> {
    msg::send_and_wait_for_reply(ft_program_id, FTAction::Transfer { from, to, amount }, 0).unwrap()
}

fn get(contracts: &mut BTreeMap<u128, Contract>, contract_id: u128) -> &mut Contract {
    if let Some(contract) = contracts.get_mut(&contract_id) {
        contract
    } else {
        panic!("A contract with the {contract_id} ID does not exist");
    }
}

#[derive(Default)]
struct Escrow {
    ft_program_id: ActorId,
    contracts: BTreeMap<u128, Contract>,
    id_nonce: u128,
}

impl Escrow {
    /// Creates one escrow contract and replies with an ID of this created contract.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer or seller for this contract.
    ///
    /// Arguments:
    /// * `buyer`: a buyer.
    /// * `seller`: a seller.
    /// * `amount`: an amount of tokens.
    fn create(&mut self, buyer: ActorId, seller: ActorId, amount: u128) {
        if msg::source() != buyer && msg::source() != seller {
            panic!("msg::source() must be a buyer or seller to create this contract");
        }

        let contract_id = self.id_nonce;
        self.id_nonce += 1;

        self.contracts.insert(
            contract_id,
            Contract {
                buyer,
                seller,
                amount,
                state: State::AwaitingDeposit,
            },
        );

        msg::reply(EscrowEvent::Created { contract_id }, 0).unwrap();
    }

    /// Makes a deposit from a buyer to an escrow account
    /// and changes a contract state to `AwaitingConfirmation`.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer saved in a contract.
    /// * Contract must not be paid or completed.
    ///
    /// Arguments:
    /// * `contract_id`: a contract ID.
    async fn deposit(&mut self, contract_id: u128) {
        let contract = get(&mut self.contracts, contract_id);

        if msg::source() != contract.buyer {
            panic!("msg::source() must a buyer saved in a contract to make a deposit");
        }

        if contract.state != State::AwaitingDeposit {
            panic!("Contract can't take deposit if it's paid or completed");
        }

        transfer_tokens(
            self.ft_program_id,
            contract.buyer,
            exec::program_id(),
            contract.amount,
        )
        .await
        .expect("Error when taking a deposit");

        contract.state = State::AwaitingConfirmation;

        msg::reply(
            EscrowEvent::Deposited {
                buyer: contract.buyer,
                amount: contract.amount,
            },
            0,
        )
        .unwrap();
    }

    /// Confirms contract by transferring tokens from an escrow account
    /// to a seller and changing contract state to `Completed`.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer saved in contract.
    /// * Contract must be paid and uncompleted.
    ///
    /// Arguments:
    /// * `contract_id`: a contract ID.
    async fn confirm(&mut self, contract_id: u128) {
        let contract = get(&mut self.contracts, contract_id);

        if msg::source() != contract.buyer {
            panic!("msg::source() must a buyer saved in a contract to confirm it")
        }

        if contract.state != State::AwaitingConfirmation {
            panic!("Contract can't be confirmed if it's not paid or completed");
        }

        transfer_tokens(
            self.ft_program_id,
            exec::program_id(),
            contract.seller,
            contract.amount,
        )
        .await
        .expect("Error when confirming a contract");

        contract.state = State::Completed;

        msg::reply(
            EscrowEvent::Confirmed {
                amount: contract.amount,
                seller: contract.seller,
            },
            0,
        )
        .unwrap();
    }

    /// Refunds tokens from an escrow account to a buyer
    /// and changes contract state to `AwaitingDeposit`
    /// (that is, a contract can be reused).
    ///
    /// Requirements:
    /// * `msg::source()` must be a seller saved in contract.
    /// * Contract must be paid and uncompleted.
    ///
    /// Arguments:
    /// * `contract_id`: a contract ID.
    async fn refund(&mut self, contract_id: u128) {
        let contract = get(&mut self.contracts, contract_id);

        if msg::source() != contract.seller {
            panic!("msg::source() must be a seller saved in contract to refund")
        }

        if contract.state != State::AwaitingConfirmation {
            panic!("Contract can't be refunded if it's not paid or completed");
        }

        transfer_tokens(
            self.ft_program_id,
            exec::program_id(),
            contract.buyer,
            contract.amount,
        )
        .await
        .expect("Error when refunding a contract");

        contract.state = State::AwaitingDeposit;

        msg::reply(
            EscrowEvent::Refunded {
                amount: contract.amount,
                buyer: contract.buyer,
            },
            0,
        )
        .unwrap();
    }

    /// Cancels (early completes) a contract by changing its state to `Completed`.
    ///
    /// Requirements:
    /// * `msg::source()` must be a buyer or seller saved in contract.
    /// * Contract must not be paid or completed.
    ///
    /// Arguments:
    /// * `contract_id`: a contract ID.
    async fn cancel(&mut self, contract_id: u128) {
        let contract = get(&mut self.contracts, contract_id);

        if msg::source() != contract.buyer && msg::source() != contract.seller {
            panic!("msg::source() must be a buyer or seller saved in contract to cancel it");
        }

        if contract.state != State::AwaitingDeposit {
            panic!("Contract can't be cancelled if it's paid or completed");
        }

        contract.state = State::Completed;

        msg::reply(
            EscrowEvent::Cancelled {
                buyer: contract.buyer,
                seller: contract.seller,
                amount: contract.amount,
            },
            0,
        )
        .unwrap();
    }
}

struct Contract {
    buyer: ActorId,
    seller: ActorId,
    state: State,
    amount: u128,
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
        EscrowAction::Deposit { contract_id } => escrow.deposit(contract_id).await,
        EscrowAction::Confirm { contract_id } => escrow.confirm(contract_id).await,
        EscrowAction::Refund { contract_id } => escrow.refund(contract_id).await,
        EscrowAction::Cancel { contract_id } => escrow.cancel(contract_id).await,
    }
}
