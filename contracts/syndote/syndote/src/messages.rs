use crate::*;
use gstd::errors::ContractError;

pub async fn take_your_turn(player: &ActorId, game: &Game) -> Result<Vec<u8>, ContractError> {
    let players = game
        .players
        .iter()
        .map(|(key, value)| (*key, value.clone()))
        .collect();
    msg::send_for_reply(
        *player,
        YourTurn {
            players,
            properties: game.properties.clone(),
        },
        0,
    )
    .expect("Error on sending `YourTurn` message")
    .up_to(Some(WAIT_DURATION))
    .expect("Invalid wait duration.")
    .await
}
