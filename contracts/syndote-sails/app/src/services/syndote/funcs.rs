

pub fn register(storage: &mut Storage, player: &ActorId) {
    storage.check_status(GameStatus::Registration);
    assert!(
        !self.players.contains_key(player),
        "You have already registered"
    );
    self.players.insert(
        *player,
        PlayerInfo {
            balance: INITIAL_BALANCE,
            ..Default::default()
        },
    );
    self.players_queue.push(*player);
    if self.players_queue.len() == NUMBER_OF_PLAYERS as usize {
        self.game_status = GameStatus::Play;
    }
    msg::reply(GameEvent::Registered, 0)
        .expect("Error in sending a reply `GameEvent::Registered`");
}
