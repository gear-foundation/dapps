#![allow(static_mut_refs)]
use sails_rs::collections::{HashMap, HashSet};
use sails_rs::gstd::{exec, msg};
use sails_rs::prelude::*;
use utils::*;
mod curve;
mod utils;
pub mod verify;
use crate::services::game::curve::{
    calculate_agg_pub_key, decrypt_point, get_cards_and_decryptions, get_decrypted_points,
    init_deck_and_card_map, substract_agg_pub_key, verify_cards,
};
use crate::services::session::Storage as SessionStorage;
use ark_ed_on_bls12_381_bandersnatch::EdwardsProjective;
use pts_client::pts::io as pts_io;
use zk_verification_client::zk_verification::io as zk_io;

pub use verify::ShuffleChainValidator;

use zk_verification_client::VerificationVariables;

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Hash)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct EncryptedCard {
    pub c0: [Vec<u8>; 3], // (x, y, z)
    pub c1: [Vec<u8>; 3], // (x, y, z)
}

#[derive(Debug)]
struct Storage {
    // for zk
    zk_verification_id: ActorId,
    encrypted_deck: Option<Vec<EncryptedCard>>,
    encrypted_cards: HashMap<ActorId, [EncryptedCard; 2]>,
    partially_decrypted_cards: HashMap<ActorId, [EncryptedCard; 2]>,
    partial_table_card_decryptions: HashMap<EncryptedCard, PartialDecryptionsByCard>,
    revealed_table_cards: Vec<Card>,
    original_card_map: HashMap<EdwardsProjective, Card>,
    original_deck: Vec<EdwardsProjective>,
    table_cards: Vec<EncryptedCard>,
    deck_position: usize,
    participants: Vec<(ActorId, Participant)>,
    waiting_participants: Vec<(ActorId, Participant)>,
    agg_pub_key: ZkPublicKey,
    // active_participants - players who can place bets
    // not to be confused with those who are in the game, as there are also all in players.
    active_participants: TurnManager<ActorId>,
    revealed_players: HashMap<ActorId, (Card, Card)>,
    status: Status,
    config: Config,
    round: u64,
    betting: Option<BettingStage>,
    betting_bank: HashMap<ActorId, u128>,
    all_in_players: Vec<ActorId>,
    already_invested_in_the_circle: HashMap<ActorId, u128>, // The mapa is needed to keep track of how much a person has put on the table,
    // which can change after each player's turn
    pts_actor_id: ActorId,
    factory_actor_id: ActorId,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Status {
    Registration,
    WaitingShuffleVerification,
    WaitingStart,
    WaitingPartialDecryptionsForPlayersCards,
    Play { stage: Stage },
    WaitingForCardsToBeDisclosed,
    WaitingForAllTableCardsToBeDisclosed,
    Finished { pots: Vec<(u128, Vec<ActorId>)> },
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Action {
    Fold,
    Call,
    Raise { bet: u128 },
    Check,
    AllIn,
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Config {
    pub admin_id: ActorId,
    admin_name: String,
    lobby_name: String,
    small_blind: u128,
    big_blind: u128,
    starting_bank: u128,
    time_per_move_ms: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Participant {
    name: String,
    balance: u128,
    pk: ZkPublicKey,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq, Hash)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ZkPublicKey {
    pub x: [u8; 32],
    pub y: [u8; 32],
    pub z: [u8; 32],
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    Registered {
        participant_id: ActorId,
        pk: ZkPublicKey,
    },
    PlayerDeleted {
        player_id: ActorId,
    },
    RegistrationCanceled {
        player_id: ActorId,
    },
    DeckShuffleComplete,
    GameStarted,
    CardsDealtToPlayers(Vec<(ActorId, [EncryptedCard; 2])>),
    CardsDealtToTable(Vec<EncryptedCard>),
    GameRestarted {
        status: Status,
    },
    SmallBlindIsSet,
    BigBlindIsSet,
    TurnIsMade {
        action: Action,
    },
    NextStage(Stage),
    Finished {
        pots: Vec<(u128, Vec<ActorId>)>,
    },
    Killed,
    AllPartialDecryptionsSubmited,
    TablePartialDecryptionsSubmited,
    CardsDisclosed,
    GameCanceled,
    WaitingForCardsToBeDisclosed,
    WaitingForAllTableCardsToBeDisclosed,
    RegisteredToTheNextRound {
        participant_id: ActorId,
        pk: ZkPublicKey,
    },
}

pub struct PokerService(());

impl PokerService {
    pub fn init(
        config: Config,
        pts_actor_id: ActorId,
        pk: ZkPublicKey,
        zk_verification_id: ActorId,
    ) -> Self {
        let participants = vec![(
            config.admin_id,
            Participant {
                name: config.admin_name.clone(),
                balance: config.starting_bank,
                pk: pk.clone(),
            },
        )];
        let mut active_participants = TurnManager::new();
        active_participants.add(config.admin_id);

        let (original_deck, original_card_map) = init_deck_and_card_map();
        unsafe {
            STORAGE = Some(Storage {
                zk_verification_id,
                config,
                status: Status::Registration,
                participants,
                waiting_participants: Vec::new(),
                active_participants,
                round: 0,
                betting: None,
                betting_bank: HashMap::new(),
                all_in_players: Vec::new(),
                already_invested_in_the_circle: HashMap::new(),
                encrypted_deck: None,
                deck_position: 0,
                encrypted_cards: HashMap::new(),
                table_cards: Vec::new(),
                partially_decrypted_cards: HashMap::new(),
                revealed_table_cards: Vec::new(),
                original_card_map,
                original_deck,
                partial_table_card_decryptions: HashMap::new(),
                pts_actor_id,
                factory_actor_id: msg::source(),
                agg_pub_key: pk,
                revealed_players: HashMap::new(),
            });
        }
        Self(())
    }
    fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

async fn pts_transfer(pts_actor_id: ActorId, from: ActorId, to: ActorId, amount: u128) {
    let request = pts_io::Transfer::encode_call(from, to, amount);

    msg::send_bytes_for_reply(pts_actor_id, request, 0, 0)
        .expect("Error in async message to PTS contract")
        .await
        .expect("PTS: Error transfer points to player");
}

fn process_blind(storage: &mut Storage, player_id: ActorId, blind_amount: u128) {
    let (_, participant) = storage
        .participants
        .iter_mut()
        .find(|(id, _)| *id == player_id)
        .expect("Participant not found");

    let amount = participant.balance.min(blind_amount);

    if amount < blind_amount {
        storage.active_participants.remove(&player_id);
        storage.all_in_players.push(player_id);
    }

    *storage
        .already_invested_in_the_circle
        .entry(player_id)
        .or_default() += amount;

    *storage.betting_bank.entry(player_id).or_default() += amount;

    participant.balance -= amount;
}

async fn remove_participant_if_registered(
    storage: &mut Storage,
    player_id: ActorId,
) -> Option<u128> {
    // The main list of participants
    if let Some((_, participant)) = storage.participants.iter().find(|(id, _)| *id == player_id) {
        match storage.status {
            Status::Registration
            | Status::WaitingShuffleVerification
            | Status::WaitingStart
            | Status::Finished { .. } => (),
            _ => panic!("Wrong status"),
        }

        let balance = participant.balance;

        storage.participants.retain(|(id, _)| *id != player_id);
        storage
            .active_participants
            .remove_and_update_first_index(&player_id);

        storage.status = Status::Registration;
        return Some(balance);
    }

    // Waiting participants
    if let Some((_, participant)) = storage
        .waiting_participants
        .iter()
        .find(|(id, _)| *id == player_id)
    {
        let balance = participant.balance;
        storage
            .waiting_participants
            .retain(|(id, _)| *id != player_id);
        return Some(balance);
    }

    None
}

impl Storage {
    fn reset_for_new_game(&mut self) {
        self.encrypted_deck = None;
        self.deck_position = 0;
        self.encrypted_cards = HashMap::new();
        self.table_cards = Vec::new();
        self.partially_decrypted_cards = HashMap::new();
        self.revealed_table_cards = Vec::new();
        self.revealed_players = HashMap::new();
        self.partial_table_card_decryptions = HashMap::new();
        self.betting_bank = HashMap::new();
        self.all_in_players = Vec::new();
        self.already_invested_in_the_circle = HashMap::new();
        self.betting = None;
    }

    pub fn refund_bets_to_players(&mut self) {
        for (id, bet) in &self.betting_bank {
            if *bet != 0 {
                let (_, participant) = self
                    .participants
                    .iter_mut()
                    .find(|(player_id, _)| player_id == id)
                    .expect("There is no such participant");
                participant.balance += *bet;
            }
        }
    }
}

fn get_player(session_for_account: &Option<ActorId>) -> ActorId {
    let msg_src = msg::source();
    let sessions = SessionStorage::get_session_map();
    match session_for_account {
        Some(account) => {
            let session = sessions
                .get(account)
                .expect("This account has no valid session");
            assert!(
                session.expires > exec::block_timestamp(),
                "The session has already expired"
            );
            assert_eq!(
                session.key, msg_src,
                "The account is not approved for this session"
            );
            *account
        }
        None => msg_src,
    }
}

#[sails_rs::service(events = Event)]
#[allow(clippy::new_without_default)]
impl PokerService {
    pub fn new() -> Self {
        Self(())
    }
    /// Registers a player by sending a transfer request to the PTS contract (starting_bank points).
    ///
    /// Panics if:
    /// - status is not `Registration`;
    /// - player is already registered.
    ///
    /// Sends a message to the PTS contract (pts_actor_id) to transfer points to this contract.
    /// On success, updates participant data and emits a `Registered` event.
    pub async fn register(
        &mut self,
        player_name: String,
        pk: ZkPublicKey,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        let player_id = get_player(&session_for_account);
        if storage.participants.iter().any(|(id, _)| *id == player_id) {
            panic!("Already registered");
        }

        if storage.participants.len() == 9 {
            panic!("Alerady max amount of players");
        }

        pts_transfer(
            storage.pts_actor_id,
            player_id,
            exec::program_id(),
            storage.config.starting_bank,
        )
        .await;

        let participant = Participant {
            name: player_name,
            balance: storage.config.starting_bank,
            pk: pk.clone(),
        };
        storage.agg_pub_key = calculate_agg_pub_key(&storage.agg_pub_key, &pk);

        match storage.status {
            Status::Registration => {
                storage.participants.push((player_id, participant));
                storage.active_participants.add(player_id);

                self.emit_event(Event::Registered {
                    participant_id: player_id,
                    pk,
                })
                .expect("Event Invocation Error");
            }
            _ => {
                storage.waiting_participants.push((player_id, participant));

                self.emit_event(Event::RegisteredToTheNextRound {
                    participant_id: player_id,
                    pk,
                })
                .expect("Event Invocation Error");
            }
        }
    }

    /// Cancels player registration and refunds their balance via PTS contract.
    ///
    /// Panics if:
    /// - current status is invalid for cancellation;
    /// - caller is not a registered player.
    ///
    /// Sends a transfer request to PTS contract to return points to the player.
    /// Removes player data and emits `RegistrationCanceled` event on success.
    pub async fn cancel_registration(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let player_id = get_player(&session_for_account);

        if player_id == storage.config.admin_id {
            panic!("Access denied");
        }

        if let Some((_, participant)) = storage.participants.iter().find(|(id, _)| *id == player_id)
        {
            storage.agg_pub_key = substract_agg_pub_key(&storage.agg_pub_key, &participant.pk);
        }
        if let Some(balance) = remove_participant_if_registered(storage, player_id).await {
            pts_transfer(storage.pts_actor_id, exec::program_id(), player_id, balance).await;

            self.emit_event(Event::RegistrationCanceled { player_id })
                .expect("Event Error");
        } else {
            panic!("You are not registered");
        }
    }

    /// Restarts the game, resetting status and refunding bets (if not Finished).
    /// Panics if caller is not admin.
    /// Resets game to WaitingShuffleVerification (if full) or Registration status.
    /// Emits GameRestarted event with new status.
    pub fn restart_game(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let player_id = get_player(&session_for_account);
        if player_id != storage.config.admin_id {
            panic!("Access denied");
        }
        if !matches!(storage.status, Status::Finished { .. }) {
            storage.refund_bets_to_players();
        }

        storage.reset_for_new_game();

        storage.participants.retain(|(id, info)| {
            if info.balance == 0 {
                storage.agg_pub_key = substract_agg_pub_key(&storage.agg_pub_key, &info.pk);
                self.emit_event(Event::RegistrationCanceled { player_id: *id })
                    .expect("Event Error");
                return false;
            }
            true
        });

        storage.active_participants.clear_all();
        storage
            .participants
            .append(&mut storage.waiting_participants);

        for (id, _) in storage.participants.iter() {
            storage.active_participants.add(*id);
        }

        storage.status = Status::Registration;

        self.emit_event(Event::GameRestarted {
            status: storage.status.clone(),
        })
        .expect("Event Invocation Error");
    }

    /// Admin-only function to terminate the lobby and refund all players.
    ///
    /// Panics if:
    /// - caller is not admin
    /// - wrong game status (not Registration/WaitingShuffleVerification/Finished/WaitingStart)
    ///
    /// Performs:
    /// 1. Batch transfer of all player balances via PTS contract
    /// 2. Sends DeleteLobby request to PokerFactory
    /// 3. Emits Killed event and transfers remaining funds to admin
    ///
    /// WARNING: Irreversible operation
    pub async fn kill(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get();
        let player_id = get_player(&session_for_account);
        if player_id != storage.config.admin_id {
            panic!("Access denied");
        }
        match storage.status {
            Status::Registration
            | Status::WaitingShuffleVerification
            | Status::Finished { .. }
            | Status::WaitingStart => {}
            _ => {
                panic!("Wrong status");
            }
        }
        let mut ids = Vec::new();
        let mut points = Vec::new();

        for (id, participant) in storage.participants.iter() {
            ids.push(*id);
            points.push(participant.balance);
        }
        let request = pts_io::BatchTransfer::encode_call(exec::program_id(), ids, points);

        msg::send_bytes_for_reply(storage.pts_actor_id, request, 0, 0)
            .expect("Error in async message to PTS contract")
            .await
            .expect("PTS: Error batch transfer points to players");

        let request = [
            "PokerFactory".encode(),
            "DeleteLobby".to_string().encode(),
            (exec::program_id()).encode(),
        ]
        .concat();

        msg::send_bytes_for_reply(storage.factory_actor_id, request, 0, 0)
            .expect("Error in sending message to PokerFactory")
            .await
            .expect("PokerFactory: Error DeleteLobby");

        self.emit_event(Event::Killed).expect("Notification Error");
        exec::exit(storage.config.admin_id);
    }

    pub async fn cancel_game(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        let player_id = get_player(&session_for_account);
        if player_id != storage.config.admin_id {
            panic!("Access denied");
        }
        match storage.status {
            Status::Registration | Status::Finished { .. } => {
                panic!("Wrong status");
            }
            _ => {
                storage.refund_bets_to_players();
                storage.reset_for_new_game();
                storage.status = Status::Registration;
            }
        }

        self.emit_event(Event::GameCanceled)
            .expect("Notification Error");
    }

    /// Admin-only function to forcibly remove a player and refund their balance.
    ///
    /// Panics if:
    /// - caller is not admin or tries to delete themselves
    /// - wrong game status (not Registration/WaitingShuffleVerification)
    /// - player doesn't exist
    ///
    /// Performs:
    /// 1. Transfers player's balance back to user via PTS contract
    /// 2. Removes player from all participant lists
    /// 3. Resets status to Registration
    /// 4. Emits PlayerDeleted event
    pub async fn delete_player(
        &mut self,
        player_id: ActorId,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        if get_player(&session_for_account) != storage.config.admin_id
            || player_id == storage.config.admin_id
        {
            panic!("Access denied");
        }
        if storage.status != Status::Registration
            && storage.status != Status::WaitingShuffleVerification
            && storage.status != Status::WaitingStart
        {
            panic!("Wrong status");
        }

        if let Some((_, participant)) = storage.participants.iter().find(|(id, _)| *id == player_id)
        {
            pts_transfer(
                storage.pts_actor_id,
                exec::program_id(),
                player_id,
                participant.balance,
            )
            .await;

            storage.agg_pub_key = substract_agg_pub_key(&storage.agg_pub_key, &participant.pk);
            storage.participants.retain(|(id, _)| *id != player_id);
            storage
                .active_participants
                .remove_and_update_first_index(&player_id);
            storage.status = Status::Registration;
        } else {
            panic!("There is no such player");
        }

        self.emit_event(Event::PlayerDeleted { player_id })
            .expect("Event Invocation Error");
    }

    pub async fn shuffle_deck(
        &mut self,
        encrypted_deck: Vec<EncryptedCard>,
        instances: Vec<VerificationVariables>,
    ) {
        let storage = self.get_mut();
        if storage.status != Status::WaitingShuffleVerification {
            panic!("Wrong status");
        }

        ShuffleChainValidator::validate_shuffle_chain(
            &instances,
            &storage.original_deck,
            &storage.agg_pub_key,
            &encrypted_deck,
        );

        let request = zk_io::VerifyShuffle::encode_call(instances);
        msg::send_bytes_for_reply(storage.zk_verification_id, request, 0, 0)
            .expect("Error in async message to ZK contract")
            .await
            .expect("PTS: Error ZK shuffle verification");

        storage.status = Status::WaitingPartialDecryptionsForPlayersCards;
        storage.encrypted_deck = Some(encrypted_deck);

        self.deal_player_cards();
        self.deal_table_cards(5);

        self.emit_event(Event::DeckShuffleComplete)
            .expect("Event Invocation Error");
    }

    /// Admin-only function to start the poker game after setup.
    ///
    /// Panics if:
    /// - caller is not admin
    /// - wrong status (not WaitingStart)
    ///
    /// Performs:
    /// 1. Processes small/big blinds (handles all-in cases)
    /// 2. Initializes betting stage
    /// 3. Updates game status and emits GameStarted event
    ///
    /// Note: Handles edge cases where players can't cover blinds
    pub async fn start_game(&mut self, session_for_account: Option<ActorId>) {
        let storage = self.get_mut();
        if get_player(&session_for_account) != storage.config.admin_id {
            panic!("Access denied");
        }
        if storage.participants.len() < 2 {
            panic!("Not enough participants");
        }
        if storage.status != Status::Registration {
            panic!("Wrong status");
        }

        storage.active_participants.set_first_index();

        let sb_player = storage
            .active_participants
            .next()
            .expect("No small blind player");
        process_blind(storage, sb_player, storage.config.small_blind);

        let bb_player = storage
            .active_participants
            .next()
            .expect("No big blind player");
        process_blind(storage, bb_player, storage.config.big_blind);

        storage.betting = Some(BettingStage {
            turn: storage
                .active_participants
                .next()
                .expect("The player must exist"),
            last_active_time: None,
            current_bet: storage.config.big_blind,
            acted_players: vec![],
        });

        storage.status = Status::WaitingShuffleVerification;
        storage.active_participants.new_round();
        storage.round += 1;

        self.emit_event(Event::GameStarted)
            .expect("Event Invocation Error");
    }

    fn deal_player_cards(&mut self) {
        let storage = self.get_mut();
        let deck = storage.encrypted_deck.as_ref().expect("No encrypted deck");
        let mut pos = storage.deck_position;

        let mut dealt = Vec::new();
        for id in storage.participants.iter().map(|(id, _)| id) {
            if pos + 2 > deck.len() {
                panic!("Not enough cards");
            }

            let card1 = deck[pos].clone();
            let card2 = deck[pos + 1].clone();

            storage
                .encrypted_cards
                .insert(*id, [card1.clone(), card2.clone()]);

            dealt.push((*id, [card1, card2]));

            pos += 2;
        }

        storage.deck_position = pos;
        self.emit_event(Event::CardsDealtToPlayers(dealt))
            .expect("Event Invocation Error");
    }

    pub async fn submit_all_partial_decryptions(&mut self, instances: Vec<VerificationVariables>) {
        let storage = self.get_mut();

        let cards_by_player = get_decrypted_points(&instances, &storage.encrypted_cards);

        let request = zk_io::VerifyDecrypt::encode_call(instances);
        msg::send_bytes_for_reply(storage.zk_verification_id, request, 0, 0)
            .expect("Error in async message to ZK contract")
            .await
            .expect("PTS: Error ZK decrypt verification");

        for (player, cards) in cards_by_player {
            storage.partially_decrypted_cards.insert(player, cards);
        }

        storage.status = Status::Play {
            stage: Stage::PreFlop,
        };
        if let Some(betting) = &mut storage.betting {
            betting.last_active_time = Some(exec::block_timestamp());
        }

        self.emit_event(Event::AllPartialDecryptionsSubmited)
            .expect("Event Invocation Error");
    }

    pub async fn submit_table_partial_decryptions(
        &mut self,
        instances: Vec<VerificationVariables>,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        let player_id = get_player(&session_for_account);
        let (base_index, expected_count, next_stage) = match &storage.status {
            Status::Play { stage } => match stage {
                Stage::WaitingTableCardsAfterPreFlop => (0, 3, Some(Stage::Flop)),
                Stage::WaitingTableCardsAfterFlop => (3, 1, Some(Stage::Turn)),
                Stage::WaitingTableCardsAfterTurn => (4, 1, Some(Stage::River)),
                _ => panic!("Wrong stage"),
            },
            Status::WaitingForAllTableCardsToBeDisclosed => {
                match storage.revealed_table_cards.len() {
                    0 => (0, 5, None),
                    3 => (3, 2, None),
                    4 => (4, 1, None),
                    _ => panic!("Wrong amount of revealed cards"),
                }
            }
            _ => panic!("Wrong status"),
        };

        if instances.len() != expected_count {
            panic!("Wrong amount of proofs");
        }
        let decryptions = get_cards_and_decryptions(&storage.table_cards, &instances);

        let request = zk_io::VerifyDecrypt::encode_call(instances);
        msg::send_bytes_for_reply(storage.zk_verification_id, request, 0, 0)
            .expect("Error in async message to ZK contract")
            .await
            .expect("PTS: Error ZK decrypt verification");

        if !storage.participants.iter().any(|(id, _)| *id == player_id) {
            panic!("Not participant");
        }

        if decryptions.len() != expected_count {
            panic!("Wrong count");
        }

        let expected_cards = &storage.table_cards[base_index..base_index + expected_count];

        for (card, decryption) in decryptions {
            assert!(expected_cards.contains(&card), "Wrong card");
            storage
                .partial_table_card_decryptions
                .entry(card)
                .or_default()
                .add(player_id, decryption);
        }

        let first_card = &storage.table_cards[base_index];

        let all_submitted = storage
            .partial_table_card_decryptions
            .get(first_card)
            .map(|by_card| by_card.participants.len() == storage.participants.len())
            .unwrap_or(false);
        if all_submitted {
            let mut revealed_cards = Vec::with_capacity(expected_count);
            for i in base_index..base_index + expected_count {
                let encrypted_card = &storage.table_cards[i];

                let by_card = storage
                    .partial_table_card_decryptions
                    .get(encrypted_card)
                    .expect("Decryptions must exist for this card");

                let partials = by_card.partials.to_vec();

                if let Some(card) =
                    decrypt_point(&storage.original_card_map, encrypted_card, partials)
                {
                    revealed_cards.push(card);
                } else {
                    panic!("Failed to decrypt card");
                }
            }

            storage.revealed_table_cards.extend(revealed_cards);

            if let Some(next_stage) = next_stage {
                storage.status = Status::Play { stage: next_stage };
            } else {
                storage.status = Status::WaitingForCardsToBeDisclosed;
            };

            if let Some(betting) = &mut storage.betting {
                betting.last_active_time = Some(exec::block_timestamp());
            }
        }

        self.emit_event(Event::TablePartialDecryptionsSubmited)
            .expect("Event Invocation Error");
    }

    /// Processes player actions during betting rounds.
    ///
    /// Panics if:
    /// - Wrong game status
    /// - Not player's turn
    /// - Invalid action (e.g. check when bet exists)
    ///
    /// Handles:
    /// - Fold/Call/Check/Raise/AllIn actions
    /// - Turn timers and skips
    /// - Game end conditions (single player left)
    /// - Stage transitions
    ///
    /// Emits TurnIsMade and NextStage events
    pub fn turn(&mut self, action: Action, session_for_account: Option<ActorId>) {
        let player = get_player(&session_for_account);
        let storage = self.get_mut();

        let Status::Play { stage } = &mut storage.status else {
            panic!("Wrong status");
        };

        if matches!(
            stage,
            Stage::WaitingTableCardsAfterPreFlop
                | Stage::WaitingTableCardsAfterFlop
                | Stage::WaitingTableCardsAfterTurn
        ) {
            panic!("Wrong stage");
        }

        let betting = storage.betting.as_mut().expect("No betting");

        let (_, participant) = storage
            .participants
            .iter_mut()
            .find(|(id, _)| *id == player)
            .expect("There is no such participant");

        let last_active_time = betting.last_active_time.expect("No last active time");
        let current_time = exec::block_timestamp();
        let number_of_passes = (current_time - last_active_time) / storage.config.time_per_move_ms;

        if number_of_passes != 0 {
            if let Some(next_or_last) = storage
                .active_participants
                .skip_and_remove(number_of_passes)
            {
                if storage.active_participants.len() <= 1 {
                    let prize = storage.betting_bank.values().sum();
                    participant.balance += prize;
                    storage.status = Status::Finished {
                        pots: vec![(prize, vec![next_or_last])],
                    };
                    storage.betting = None;
                    self.emit_event(Event::Finished {
                        pots: vec![(prize, vec![next_or_last])],
                    })
                    .expect("Event Error");
                    return;
                } else if next_or_last != player {
                    panic!("Not your turn!");
                }
            } else {
                panic!("No active players");
            }
        } else if betting.turn != player {
            panic!("Not your turn!");
        }
        // Process the player's action
        match action {
            Action::Fold => {
                storage.active_participants.remove(&player);
            }
            Action::Call => {
                let already_invested = *storage
                    .already_invested_in_the_circle
                    .get(&player)
                    .unwrap_or(&0);
                let call_value = betting.current_bet - already_invested;
                if call_value == 0 || participant.balance <= call_value {
                    panic!("Wrong action");
                }
                participant.balance -= call_value;
                betting.acted_players.push(player);
                storage
                    .already_invested_in_the_circle
                    .entry(player)
                    .and_modify(|v| *v += call_value)
                    .or_insert(call_value);
                storage
                    .betting_bank
                    .entry(player)
                    .and_modify(|v| *v += call_value)
                    .or_insert(call_value);
            }
            Action::Check => {
                let already_invested = *storage
                    .already_invested_in_the_circle
                    .get(&player)
                    .unwrap_or(&0);

                if betting.current_bet != already_invested {
                    panic!("cannot check");
                }

                betting.acted_players.push(player);
            }
            Action::Raise { bet } => {
                let already_invested = *storage
                    .already_invested_in_the_circle
                    .get(&player)
                    .unwrap_or(&0);

                if participant.balance <= bet {
                    panic!("Wrong action");
                }
                if already_invested + bet <= betting.current_bet {
                    panic!("Raise must be higher");
                }
                betting.current_bet = already_invested + bet;
                participant.balance -= bet;
                // if someone raises the bet, the betting round starts all over again
                // so it is necessary to clear the acted_players
                betting.acted_players.clear();
                betting.acted_players.push(player);
                storage
                    .already_invested_in_the_circle
                    .entry(player)
                    .and_modify(|v| *v += bet)
                    .or_insert(bet);
                storage
                    .betting_bank
                    .entry(player)
                    .and_modify(|v| *v += bet)
                    .or_insert(bet);
            }
            Action::AllIn => {
                let already_invested = *storage
                    .already_invested_in_the_circle
                    .get(&player)
                    .unwrap_or(&0);
                let bet = already_invested + participant.balance;
                if bet > betting.current_bet {
                    betting.current_bet = bet;
                    betting.acted_players.clear();
                }

                storage.all_in_players.push(player);
                // if a player has made a all in, we remove him from the active_participants, so that he no longer participates in bets
                storage.active_participants.remove(&player);
                storage
                    .already_invested_in_the_circle
                    .entry(player)
                    .and_modify(|v| *v += participant.balance)
                    .or_insert(participant.balance);
                storage
                    .betting_bank
                    .entry(player)
                    .and_modify(|v| *v += participant.balance)
                    .or_insert(participant.balance);
                participant.balance = 0;
            }
        }

        // Check if the game should end immediately (only one player left)
        if storage.active_participants.len() + storage.all_in_players.len() == 1 {
            let winner = if storage.active_participants.is_empty() {
                storage
                    .all_in_players
                    .first()
                    .expect("The player must exist")
            } else {
                storage
                    .active_participants
                    .get(0)
                    .expect("The player must exist")
            };
            let prize = storage.betting_bank.values().sum();
            let (_, participant) = storage
                .participants
                .iter_mut()
                .find(|(id, _)| id == winner)
                .expect("There is no such participant");

            participant.balance += prize;
            storage.status = Status::Finished {
                pots: vec![(prize, vec![*winner])],
            };
            self.emit_event(Event::Finished {
                pots: vec![(prize, vec![*winner])],
            })
            .expect("Event Error");
        }
        // Check if the round is complete at the River stage
        else if betting.acted_players.len() == storage.active_participants.len()
            && *stage == Stage::River
        {
            storage.status = Status::WaitingForCardsToBeDisclosed;
            self.emit_event(Event::WaitingForCardsToBeDisclosed)
                .expect("Event Error");
        }
        // Check if the round is complete before River stage
        else if betting.acted_players.len() == storage.active_participants.len() {
            // if there's only one active player left, there's no point in betting any more
            // and if there's nobody active player left(everybody call AllIn), there's no point in betting any more
            if storage.active_participants.len() <= 1 {
                storage.status = Status::WaitingForAllTableCardsToBeDisclosed;
                self.emit_event(Event::WaitingForAllTableCardsToBeDisclosed)
                    .expect("Event Error");
            } else {
                storage.active_participants.reset_turn_index();
                storage.already_invested_in_the_circle = HashMap::new();
                betting.turn = storage
                    .active_participants
                    .next()
                    .expect("There is no next one");
                betting.last_active_time = None;
                betting.acted_players.clear();
                betting.current_bet = 0;

                *stage = stage.clone().next().expect("There is no next one");
                self.emit_event(Event::NextStage(stage.clone()))
                    .expect("Event Error");
            }
        } else {
            betting.turn = storage
                .active_participants
                .next()
                .expect("The player must exist");
            betting.last_active_time = Some(current_time);
        }
        self.emit_event(Event::TurnIsMade { action })
            .expect("Event Error");
    }

    fn deal_table_cards(&mut self, count: usize) {
        let storage = self.get_mut();
        let deck = storage.encrypted_deck.as_ref().expect("No shuffled deck");

        if storage.deck_position + count > deck.len() {
            panic!("Not enough cards");
        }

        let mut new_cards = Vec::new();
        for _ in 0..count {
            let card = deck[storage.deck_position].clone();
            storage.table_cards.push(card.clone());
            new_cards.push(card);
            storage.deck_position += 1;
        }

        self.emit_event(Event::CardsDealtToTable(new_cards))
            .expect("Event Error");
    }

    pub async fn card_disclosure(
        &mut self,
        instances: Vec<(Card, VerificationVariables)>,
        session_for_account: Option<ActorId>,
    ) {
        let storage = self.get_mut();
        // if storage.status != Status::WaitingForCardsToBeDisclosed {
        //     panic("Wrong status")
        // }
        let player = get_player(&session_for_account);

        let partially_decrypted_cards = storage
            .partially_decrypted_cards
            .get(&player)
            .expect("Not in game");

        verify_cards(
            partially_decrypted_cards,
            instances.clone(),
            &storage.original_card_map,
        );

        let only_proofs = vec![instances[0].1.clone(), instances[1].1.clone()];

        let request = zk_io::VerifyDecrypt::encode_call(only_proofs);
        msg::send_bytes_for_reply(storage.zk_verification_id, request, 0, 0)
            .expect("Error in async message to ZK contract")
            .await
            .expect("PTS: Error ZK decrypt verification");

        let cards = (instances[0].0.clone(), instances[1].0.clone());
        storage.revealed_players.insert(player, cards);

        let expected_players: HashSet<ActorId> = storage
            .active_participants
            .all()
            .iter()
            .chain(storage.all_in_players.iter())
            .cloned()
            .collect();
        let players: HashSet<ActorId> = storage.revealed_players.keys().cloned().collect();

        if players.is_superset(&expected_players) {
            let table_cards: [Card; 5] = match storage.revealed_table_cards.clone().try_into() {
                Ok(array) => array,
                Err(_) => unreachable!(),
            };

            let pots = evaluate_round(
                storage.revealed_players.clone(),
                table_cards,
                &storage.betting_bank,
            );

            let mut prizes_by_player: HashMap<ActorId, u128> = HashMap::new();
            for (amount, winners) in &pots {
                let share = *amount / winners.len() as u128;
                for winner in winners {
                    *prizes_by_player.entry(*winner).or_insert(0) += share;
                }
            }

            for (winner, prize) in &prizes_by_player {
                let (_, participant) = storage
                    .participants
                    .iter_mut()
                    .find(|(id, _)| id == winner)
                    .expect("There is no such participant");
                participant.balance += *prize;
            }

            storage.status = Status::Finished { pots: pots.clone() };
            self.emit_event(Event::Finished { pots })
                .expect("Event Error");
        }

        self.emit_event(Event::CardsDisclosed).expect("Event Error");
    }

    // Query
    pub fn player_cards(&self, player_id: ActorId) -> Option<[EncryptedCard; 2]> {
        self.get()
            .partially_decrypted_cards
            .get(&player_id)
            .cloned()
    }

    pub fn encrypted_table_cards(&self) -> Vec<EncryptedCard> {
        self.get().table_cards.clone()
    }

    pub fn table_cards_to_decrypt(&self) -> Vec<EncryptedCard> {
        let storage = self.get();
        let (base_index, expected_count) = match &storage.status {
            Status::Play { stage } => match stage {
                Stage::WaitingTableCardsAfterPreFlop => (0, 3),
                Stage::WaitingTableCardsAfterFlop => (3, 1),
                Stage::WaitingTableCardsAfterTurn => (4, 1),
                _ => return vec![],
            },
            Status::WaitingForAllTableCardsToBeDisclosed => {
                match storage.revealed_table_cards.len() {
                    0 => (0, 5),
                    3 => (3, 2),
                    4 => (4, 1),
                    _ => return vec![],
                }
            }
            _ => return vec![],
        };
        storage.table_cards[base_index..base_index + expected_count]
            .to_vec()
            .clone()
    }

    pub fn revealed_table_cards(&self) -> Vec<Card> {
        self.get().revealed_table_cards.clone()
    }

    pub fn participants(&self) -> Vec<(ActorId, Participant)> {
        self.get().participants.clone()
    }
    pub fn waiting_participants(&self) -> Vec<(ActorId, Participant)> {
        self.get().waiting_participants.clone()
    }
    pub fn active_participants(&self) -> &'static TurnManager<ActorId> {
        &self.get().active_participants
    }
    pub fn status(&self) -> &'static Status {
        &self.get().status
    }
    pub fn config(&self) -> &'static Config {
        &self.get().config
    }
    pub fn round(&self) -> u64 {
        self.get().round
    }
    pub fn betting(&self) -> &'static Option<BettingStage> {
        &self.get().betting
    }
    pub fn betting_bank(&self) -> Vec<(ActorId, u128)> {
        self.get().betting_bank.clone().into_iter().collect()
    }
    pub fn all_in_players(&self) -> &'static Vec<ActorId> {
        &self.get().all_in_players
    }
    pub fn already_invested_in_the_circle(&self) -> Vec<(ActorId, u128)> {
        self.get()
            .already_invested_in_the_circle
            .clone()
            .into_iter()
            .collect()
    }
    pub fn factory_actor_id(&self) -> ActorId {
        self.get().factory_actor_id
    }
    pub fn pts_actor_id(&self) -> ActorId {
        self.get().pts_actor_id
    }

    pub fn revealed_players(&self) -> Vec<(ActorId, (Card, Card))> {
        self.get().revealed_players.clone().into_iter().collect()
    }

    pub fn agg_pub_key(&self) -> ZkPublicKey {
        self.get().agg_pub_key.clone()
    }
}
