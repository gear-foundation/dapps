use concert_io::*;
use gstd::String;

mod utils;
use utils::*;

#[test]
fn create_concert() {
    let system = init_system();
    let concert_program = init_concert(&system);
    create(
        &concert_program,
        USER.into(),
        CONCERT_ID,
        NUMBER_OF_TICKETS,
        DATE,
    );
}

#[test]
fn buy_tickets() {
    let system = init_system();
    let concert_program = init_concert(&system);
    create(
        &concert_program,
        USER.into(),
        CONCERT_ID,
        NUMBER_OF_TICKETS,
        DATE,
    );

    let metadata = vec![Some(TokenMetadata {
        title: Some(String::from("SUM41_TORONTO")),
        description: Some(String::from("SUM 41 Torotno Ticket. Row 4. Seat 4.")),
        media: Some(String::from("sum41.com")),
        reference: Some(String::from("UNKNOWN")),
    })];

    buy(&concert_program, CONCERT_ID, AMOUNT, metadata, false);
}

#[test]
fn buy_tickets_failures() {
    let system = init_system();
    let concert_program = init_concert(&system);
    create(
        &concert_program,
        USER.into(),
        CONCERT_ID,
        NUMBER_OF_TICKETS,
        DATE,
    );

    // MUST FAIL since we're buying < 1 ticket
    buy(&concert_program, CONCERT_ID, 0, vec![None], true);

    // MUST FAIL since we're buying more tickets than there are
    buy(
        &concert_program,
        CONCERT_ID,
        NUMBER_OF_TICKETS + 1,
        vec![None; (NUMBER_OF_TICKETS + 1) as usize],
        true,
    );

    // MUST FAIL since metadata is not provided for all tickets
    buy(
        &concert_program,
        CONCERT_ID,
        AMOUNT + 3,
        vec![None; (AMOUNT + 1) as usize],
        true,
    );
}

#[test]
fn hold_concert() {
    let system = init_system();
    let concert_program = init_concert(&system);

    create(
        &concert_program,
        USER.into(),
        CONCERT_ID,
        NUMBER_OF_TICKETS,
        DATE,
    );

    let metadata = vec![Some(TokenMetadata {
        title: Some(String::from("SUM41_TORONTO")),
        description: Some(String::from("SUM 41 Torotno Ticket. Row 4. Seat 4.")),
        media: Some(String::from("sum41.com")),
        reference: Some(String::from("UNKNOWN")),
    })];

    buy(&concert_program, CONCERT_ID, AMOUNT, metadata, false);

    hold(&concert_program, CONCERT_ID);
}
