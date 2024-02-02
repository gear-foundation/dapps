use concert_io::*;
use gstd::{prelude::*, ActorId, Encode};
use gtest::{Program, System};
use multi_token_io::{InitMtk, TokenMetadata};

pub const USER: u64 = 193;
pub const MTK_ID: u64 = 2;
pub const CONCERT_ID: u128 = 0;
pub const TOKEN_ID: u128 = 1;
pub const NUMBER_OF_TICKETS: u128 = 100;
pub const AMOUNT: u128 = 1;
pub const DATE: u128 = 100000;

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_concert(sys: &System) -> Program<'_> {
    let concert_program = Program::current_opt(sys);
    let mtk_program = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/debug/multi_token.opt.wasm",
    );
    let res = mtk_program.send(
        USER,
        InitMtk {
            name: String::from("Multitoken for a concert"),
            symbol: String::from("MTC"),
            base_uri: String::from(""),
        },
    );

    assert!(!res.main_failed());
    assert!(!concert_program
        .send(
            USER,
            InitConcert {
                owner_id: USER.into(),
                mtk_contract: MTK_ID.into(),
            },
        )
        .main_failed());

    concert_program
}

pub fn create(
    concert_program: &Program<'_>,
    creator: ActorId,
    name: String,
    description: String,
    number_of_tickets: u128,
    date: u128,
    concert_id: u128,
) {
    let res = concert_program.send(
        USER,
        ConcertAction::Create {
            creator,
            name,
            description,
            number_of_tickets,
            date,
            token_id: TOKEN_ID,
        },
    );

    assert!(res.contains(&(
        USER,
        Ok::<ConcertEvent, ConcertError>(ConcertEvent::Creation {
            creator,
            concert_id,
            number_of_tickets,
            date,
        })
        .encode()
    )));
}

pub fn buy(
    concert_program: &Program<'_>,
    concert_id: u128,
    amount: u128,
    metadata: Vec<Option<TokenMetadata>>,
    error: Option<ConcertError>,
) {
    let res = concert_program.send(USER, ConcertAction::BuyTickets { amount, metadata });

    if let Some(error) = error {
        assert!(res.contains(&(USER, Err::<ConcertEvent, ConcertError>(error).encode())));
    } else {
        assert!(res.contains(&(
            USER,
            Ok::<ConcertEvent, ConcertError>(ConcertEvent::Purchase { concert_id, amount })
                .encode()
        )));
    }
}

pub fn hold(concert_program: &Program<'_>, concert_id: u128) {
    let res = concert_program.send(USER, ConcertAction::Hold);

    assert!(res.contains(&(
        USER,
        Ok::<ConcertEvent, ConcertError>(ConcertEvent::Hold { concert_id }).encode()
    )));
}

pub fn check_current_concert(
    concert_program: &Program<'_>,
    name: String,
    description: String,
    date: u128,
    number_of_tickets: u128,
    tickets_left: u128,
) {
    let state: State = concert_program.read_state(0).expect("Can't read state");
    let CurrentConcert {
        name: true_name,
        description: true_description,
        date: true_date,
        number_of_tickets: true_number_of_tickets,
        tickets_left: true_tickets_left,
    } = state.current_concert();
    if name != true_name {
        std::panic!("CONCERT: Concert name differs.");
    }
    if description != true_description {
        std::panic!("CONCERT: Concert description differs.");
    }
    if date != true_date {
        std::panic!("CONCERT: Concert date differs.");
    }
    if number_of_tickets != true_number_of_tickets {
        std::panic!("CONCERT: Concert number of tickets differs.");
    }
    if tickets_left != true_tickets_left {
        std::panic!("CONCERT: Concert number of tickets left differs.");
    }
}

pub fn check_user_tickets(
    concert_program: &Program<'_>,
    user: ActorId,
    tickets: Vec<Option<TokenMetadata>>,
) {
    let state: State = concert_program.read_state(0).expect("Can't read state");
    let true_tickets = state.user_tickets(user);
    if tickets != true_tickets {
        std::panic!("CONCERT: User tickets differ.");
    }
}

pub fn check_buyers(concert_program: &Program<'_>, buyers: Vec<ActorId>) {
    let state: State = concert_program.read_state(0).expect("Can't read state");
    if buyers != state.buyers {
        std::panic!("CONCERT: Buyers list differs.");
    }
}
