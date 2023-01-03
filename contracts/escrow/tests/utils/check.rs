use super::*;

pub fn create(
    escrow_program: &Program,
    wallet_id: u128,
    from: u64,
    buyer: u64,
    seller: u64,
    amount: u128,
) {
    assert!(escrow_program
        .send(
            from,
            EscrowAction::Create {
                buyer: buyer.into(),
                seller: seller.into(),
                amount,
            },
        )
        .contains(&(from, EscrowEvent::Created(wallet_id.into()).encode())));
}

pub fn deposit(escrow_program: &Program, wallet_id: u128, buyer: u64, expected_tx_id: u64) {
    assert!(escrow_program
        .send(buyer, EscrowAction::Deposit(wallet_id.into()))
        .contains(&(
            buyer,
            EscrowEvent::Deposited(expected_tx_id, wallet_id.into()).encode()
        )));
}

pub fn confirm(escrow_program: &Program, wallet_id: u128, buyer: u64, expected_tx_id: u64) {
    assert!(escrow_program
        .send(buyer, EscrowAction::Confirm(wallet_id.into()))
        .contains(&(
            buyer,
            EscrowEvent::Confirmed(expected_tx_id, wallet_id.into()).encode()
        )));
}

pub fn refund(escrow_program: &Program, wallet_id: u128, seller: u64, expected_tx_id: u64) {
    assert!(escrow_program
        .send(seller, EscrowAction::Refund(wallet_id.into()))
        .contains(&(
            seller,
            EscrowEvent::Refunded(expected_tx_id, wallet_id.into()).encode()
        )));
}

pub fn cancel(escrow_program: &Program, wallet_id: u128, from: u64) {
    assert!(escrow_program
        .send(from, EscrowAction::Cancel(wallet_id.into()))
        .contains(&(from, EscrowEvent::Cancelled(wallet_id.into()).encode())));
}

pub fn info(escrow_program: &Program, wallet_id: u128, wallet_info: Wallet) {
    assert_eq!(
        escrow_program
            .meta_state::<_, EscrowStateReply>(EscrowState::Info(wallet_id.into()))
            .unwrap(),
        EscrowStateReply::Info(wallet_info)
    )
}

pub fn created_wallets(escrow_program: &Program, mut created_wallets: Vec<(WalletId, Wallet)>) {
    let reply = escrow_program
        .meta_state::<_, EscrowStateReply>(EscrowState::CreatedWallets)
        .unwrap();
    match reply {
        EscrowStateReply::CreatedWallets(mut vec) => {
            vec.sort_by(|(wallet_id_1, _wallet1), (wallet_id_2, _wallet2)| {
                wallet_id_1.cmp(wallet_id_2)
            });
            created_wallets.sort_by(|(wallet_id_1, _wallet1), (wallet_id_2, _wallet2)| {
                wallet_id_1.cmp(wallet_id_2)
            });

            assert_eq!(vec, created_wallets)
        }
        _ => panic!("wrong reply"),
    }
}
