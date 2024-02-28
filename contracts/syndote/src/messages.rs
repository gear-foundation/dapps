use crate::*;

pub fn take_your_turn(
    reservation_id: ReservationId,
    player: &ActorId,
    game_info: GameInfo,
) -> Result<MessageId, GameError> {
    msg::send_from_reservation(reservation_id, *player, YourTurn { game_info }, 0)
        .map_err(|_| GameError::ReservationNotValid)
}

pub fn msg_to_play_game(
    reservation_id: ReservationId,
    program_id: &ActorId,
    admin_id: &ActorId,
) -> Result<MessageId, GameError> {
    msg::send_from_reservation(
        reservation_id,
        *program_id,
        GameAction::Play {
            admin_id: *admin_id,
        },
        0,
    )
    .map_err(|_| GameError::ReservationNotValid)
}
