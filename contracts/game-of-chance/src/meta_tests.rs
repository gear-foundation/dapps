#[cfg(test)]
extern crate std;
#[cfg(test)]
use std::println;
use std::time::{SystemTime, UNIX_EPOCH};

use codec::Encode;
use gstd::BTreeMap;
use gtest::{Program, System};
use lt_io::*;
const USERS: &[u64] = &[3, 4, 5];

fn init(sys: &System) {
    sys.init_logger();

    let lt = Program::current(sys);

    let res = lt.send_bytes_with_value(USERS[0], b"Init", 10000);

    assert!(res.log().is_empty());
}

#[test]
fn meta_tests() {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let sys = System::new();
    init(&sys);
    let lt = sys.get_program(1);

    let mut players = BTreeMap::<u32, Player>::new();

    let res = lt.send(
        USERS[0],
        LtAction::StartLottery {
            duration: 5000,
            token_address: None,
            participation_cost: 1000,
            prize_fund: 2000,
        },
    );
    assert!(res.log().is_empty());

    let res = lt.send_with_value(USERS[0], LtAction::Enter(1000), 1000);
    assert!(res.contains(&(USERS[0], LtEvent::PlayerAdded(0).encode())));

    players.insert(
        0,
        Player {
            player_id: USERS[0].into(),
            balance: 1000,
        },
    );

    let res = lt.send_with_value(USERS[1], LtAction::Enter(1000), 1000);
    assert!(res.contains(&(USERS[1], LtEvent::PlayerAdded(1).encode())));

    players.insert(
        1,
        Player {
            player_id: USERS[1].into(),
            balance: 1000,
        },
    );

    println!(
        "meta players: {:?}",
        lt.meta_state::<_, LtStateReply>(LtState::GetPlayers)
    );

    assert_eq!(
        lt.meta_state::<_, LtStateReply>(LtState::GetPlayers),
        LtStateReply::Players(players.clone())
    );

    assert_eq!(
        lt.meta_state::<_, LtStateReply>(LtState::BalanceOf(0)),
        LtStateReply::Balance(1000)
    );

    assert_eq!(
        lt.meta_state::<_, LtStateReply>(LtState::LotteryState),
        LtStateReply::LotteryState {
            lottery_owner: USERS[0].into(),
            lottery_started: true,
            lottery_start_time: time,
            lottery_duration: 5000,
            participation_cost: 1000,
            prize_fund: 2000,
            token_address: None,
            lottery_id: 1,
            players,
        }
    );

    sys.spend_blocks(5000);

    let res = lt.send(USERS[0], LtAction::PickWinner);

    println!("Winner index: {:?}", res.decoded_log::<LtEvent>());
    assert!(
        res.contains(&(USERS[0], LtEvent::Winner(0).encode()))
            || res.contains(&(USERS[0], LtEvent::Winner(1).encode()))
    );

    println!(
        "meta Winners: {:?}",
        lt.meta_state::<_, LtStateReply>(LtState::GetWinners)
    );
}
