use concert_io::*;
use gstd::ActorId;
use gstd::{Decode, Encode};
use gtest::{Program, System, WasmProgram};

pub const USER: u64 = 193;
pub const MTK_ID: u64 = 2;
pub const CONCERT_ID: u128 = 1;
pub const NUMBER_OF_TICKETS: u128 = 100;
pub const AMOUNT: u128 = 1;
pub const ZERO_ID: ActorId = ActorId::new([0u8; 32]);
pub const DATE: u128 = 100000;

#[derive(Debug)]
struct MultiToken;

impl WasmProgram for MultiToken {
    fn init(&mut self, _: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        Ok(Some(b"INITIALIZED".to_vec()))
    }

    fn handle(&mut self, payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        let res = MTKAction::decode(&mut &payload[..]).map_err(|_| "Can not decode")?;
        match res {
            MTKAction::Mint {
                account,
                id,
                amount,
                meta: _,
            } => Ok(Some(
                MTKEvent::TransferSingle(TransferSingleReply {
                    operator: 1.into(),
                    from: ZERO_ID,
                    to: account,
                    id,
                    amount,
                })
                .encode(),
            )),
            MTKAction::MintBatch {
                account,
                ids,
                amounts,
                meta: _,
            } => Ok(Some(
                MTKEvent::TransferBatch {
                    operator: 1.into(),
                    from: ZERO_ID,
                    to: account,
                    ids: ids.to_vec(),
                    values: amounts.to_vec(),
                }
                .encode(),
            )),
            MTKAction::Burn { id, amount } => Ok(Some(
                MTKEvent::TransferSingle(TransferSingleReply {
                    operator: 1.into(),
                    from: 1.into(),
                    to: ZERO_ID,
                    id,
                    amount,
                })
                .encode(),
            )),
            MTKAction::BalanceOfBatch {
                accounts: _,
                ids: _,
            } => {
                let res = vec![BalanceOfBatchReply {
                    account: 1.into(),
                    id: CONCERT_ID,
                    amount: AMOUNT,
                }];
                Ok(Some(MTKEvent::BalanceOfBatch(res).encode()))
            }
        }
    }

    fn handle_reply(&mut self, _: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        Ok(None)
    }
}

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_concert(sys: &System) -> Program {
    let concert_program = Program::current(sys);
    let mtk_program = Program::mock_with_id(sys, MTK_ID, MultiToken);
    let res = mtk_program.send_bytes(100001, "INIT");
    assert!(!res.log().is_empty());
    assert!(concert_program
        .send(
            USER,
            InitConcert {
                owner_id: USER.into(),
                mtk_contract: MTK_ID.into(),
            },
        )
        .log()
        .is_empty());

    concert_program
}

pub fn create(
    concert_program: &Program,
    creator: ActorId,
    concert_id: u128,
    number_of_tickets: u128,
    date: u128,
) {
    let res = concert_program.send(
        USER,
        ConcertAction::Create {
            creator,
            concert_id,
            number_of_tickets,
            date,
        },
    );

    assert!(res.contains(&(
        USER,
        ConcertEvent::Creation {
            creator,
            concert_id,
            number_of_tickets,
            date,
        }
        .encode()
    )));
}

pub fn buy(
    concert_program: &Program,
    concert_id: u128,
    amount: u128,
    metadata: Vec<Option<TokenMetadata>>,
    should_fail: bool,
) {
    let res = concert_program.send(
        USER,
        ConcertAction::BuyTickets {
            concert_id,
            amount,
            metadata,
        },
    );

    if should_fail {
        assert!(res.main_failed());
    } else {
        assert!(res.contains(&(USER, ConcertEvent::Purchase { concert_id, amount }.encode())));
    }
}

pub fn hold(concert_program: &Program, concert_id: u128) {
    let res = concert_program.send(USER, ConcertAction::Hold { concert_id });

    assert!(res.contains(&(USER, ConcertEvent::Hold { concert_id }.encode())));
}
