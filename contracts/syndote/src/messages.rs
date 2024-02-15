use crate::*;

pub fn take_your_turn(
    reservation_id: ReservationId,
    player: &ActorId,
    game_info: GameInfo,
) -> MessageId {
    msg::send_from_reservation(reservation_id, *player, YourTurn { game_info }, 0)
        .expect("Error on sending `YourTurn` message")
}
