use super::*;

pub fn create(escrow_program: &Program, from: u64, buyer: u64, seller: u64, amount: u128) {
    assert!(escrow_program
        .send(
            from,
            EscrowAction::Create {
                buyer: buyer.into(),
                seller: seller.into(),
                amount,
            },
        )
        .main_failed());
}

pub fn deposit(escrow_program: &Program, wallet_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Deposit(wallet_id.into()))
        .main_failed());
}

pub fn confirm(escrow_program: &Program, wallet_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Confirm(wallet_id.into()))
        .main_failed());
}

pub fn refund(escrow_program: &Program, wallet_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Refund(wallet_id.into()))
        .main_failed());
}

pub fn cancel(escrow_program: &Program, wallet_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Cancel(wallet_id.into()))
        .main_failed());
}

pub fn info(escrow_program: &Program, wallet_id: u128) {
    escrow_program.meta_state::<_, EscrowStateReply>(EscrowState::Info(wallet_id.into()));
}
