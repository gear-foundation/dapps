pub mod utils;
use utils::*;

#[test]
fn two_different_escrows() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_ft(&system);

    mint(&ft_program, BUYER[0], AMOUNT[0] + AMOUNT_REMAINDER);
    mint(&ft_program, BUYER[1], AMOUNT[1] + AMOUNT_REMAINDER);

    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::create(
        &escrow_program,
        WALLET[1],
        SELLER[1],
        BUYER[1],
        SELLER[1],
        AMOUNT[1],
    );

    check::info(
        &escrow_program,
        WALLET[0],
        Wallet {
            buyer: BUYER[0].into(),
            seller: SELLER[0].into(),
            amount: AMOUNT[0],
            state: WalletState::AwaitingDeposit,
        },
    );
    check::info(
        &escrow_program,
        WALLET[1],
        Wallet {
            buyer: BUYER[1].into(),
            seller: SELLER[1].into(),
            amount: AMOUNT[1],
            state: WalletState::AwaitingDeposit,
        },
    );

    check::deposit(&escrow_program, WALLET[0], BUYER[0]);
    check::deposit(&escrow_program, WALLET[1], BUYER[1]);

    check::info(
        &escrow_program,
        WALLET[0],
        Wallet {
            buyer: BUYER[0].into(),
            seller: SELLER[0].into(),
            amount: AMOUNT[0],
            state: WalletState::AwaitingConfirmation,
        },
    );
    check::info(
        &escrow_program,
        WALLET[1],
        Wallet {
            buyer: BUYER[1].into(),
            seller: SELLER[1].into(),
            amount: AMOUNT[1],
            state: WalletState::AwaitingConfirmation,
        },
    );

    check::confirm(&escrow_program, WALLET[0], BUYER[0]);
    check::confirm(&escrow_program, WALLET[1], BUYER[1]);

    check::info(
        &escrow_program,
        WALLET[0],
        Wallet {
            buyer: BUYER[0].into(),
            seller: SELLER[0].into(),
            amount: AMOUNT[0],
            state: WalletState::Closed,
        },
    );
    check::info(
        &escrow_program,
        WALLET[1],
        Wallet {
            buyer: BUYER[1].into(),
            seller: SELLER[1].into(),
            amount: AMOUNT[1],
            state: WalletState::Closed,
        },
    );

    check_balance(&ft_program, BUYER[0], AMOUNT_REMAINDER);
    check_balance(&ft_program, BUYER[1], AMOUNT_REMAINDER);

    check_balance(&ft_program, SELLER[0], AMOUNT[0]);
    check_balance(&ft_program, SELLER[1], AMOUNT[1]);
}

#[test]
fn reuse_after_refund() {
    let system = init_system();
    let escrow_program = init_escrow(&system);
    let ft_program = init_ft(&system);

    mint(&ft_program, BUYER[0], AMOUNT[0]);
    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::deposit(&escrow_program, WALLET[0], BUYER[0]);

    check::refund(&escrow_program, WALLET[0], SELLER[0]);
    check_balance(&ft_program, BUYER[0], AMOUNT[0]);

    check::deposit(&escrow_program, WALLET[0], BUYER[0]);
    check::confirm(&escrow_program, WALLET[0], BUYER[0]);
}

#[test]
fn interact_with_non_existend_wallet() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    fail::deposit(&escrow_program, NONEXISTENT_WALLET, BUYER[0]);
    fail::cancel(&escrow_program, NONEXISTENT_WALLET, BUYER[0]);
    fail::refund(&escrow_program, NONEXISTENT_WALLET, BUYER[0]);
    fail::confirm(&escrow_program, NONEXISTENT_WALLET, BUYER[0]);
}

#[test]
#[should_panic]
fn interact_with_non_existend_wallet_meta_state() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    fail::info(&escrow_program, NONEXISTENT_WALLET);
}

#[test]
fn created_wallets() {
    let system = init_system();
    let escrow_program = init_escrow(&system);

    check::created_wallets(&escrow_program, vec![]);

    check::create(
        &escrow_program,
        WALLET[0],
        SELLER[0],
        BUYER[0],
        SELLER[0],
        AMOUNT[0],
    );
    check::create(
        &escrow_program,
        WALLET[1],
        SELLER[1],
        BUYER[1],
        SELLER[1],
        AMOUNT[1],
    );

    check::created_wallets(
        &escrow_program,
        vec![
            (
                WALLET[0].into(),
                Wallet {
                    amount: AMOUNT[0],
                    buyer: BUYER[0].into(),
                    seller: SELLER[0].into(),
                    state: WalletState::AwaitingDeposit,
                },
            ),
            (
                WALLET[1].into(),
                Wallet {
                    amount: AMOUNT[1],
                    buyer: BUYER[1].into(),
                    seller: SELLER[1].into(),
                    state: WalletState::AwaitingDeposit,
                },
            ),
        ],
    );
}
