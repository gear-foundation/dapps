use core::fmt::Debug;
use sails_rs::collections::HashMap;
use sails_rs::collections::HashSet;
use sails_rs::prelude::*;
use sails_rs::{ActorId, Vec};

type PartialDecryption = [Vec<u8>; 3];
#[derive(Default, Debug)]
pub struct PartialDecryptionsByCard {
    pub partials: Vec<PartialDecryption>,
    pub participants: HashSet<ActorId>,
}

impl PartialDecryptionsByCard {
    pub fn add(&mut self, actor: ActorId, decryption: PartialDecryption) {
        if self.participants.contains(&actor) {
            panic!("Already send decryption for this card");
        }
        self.partials.push(decryption);
        self.participants.insert(actor);
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct TurnManager<Id> {
    active_ids: Vec<Id>,
    turn_index: u64,
    first_index: u16,
}

#[allow(clippy::new_without_default)]
impl<Id: Eq + Clone + Debug> TurnManager<Id> {
    pub fn new() -> Self {
        Self {
            active_ids: Vec::new(),
            turn_index: 0,
            first_index: 0,
        }
    }

    pub fn new_round(&mut self) {
        self.first_index = (self.first_index + 1) % self.active_ids.len() as u16;
    }

    pub fn add(&mut self, id: Id) {
        self.active_ids.push(id.clone());
    }

    pub fn remove(&mut self, id: &Id) {
        if let Some(pos) = self.active_ids.iter().position(|x| x == id) {
            self.active_ids.remove(pos);

            if self.turn_index as usize > pos {
                self.turn_index -= 1;
            } else if self.turn_index as usize >= self.active_ids.len() {
                self.turn_index = 0;
            }
        }
    }

    pub fn next(&mut self) -> Option<Id> {
        if self.active_ids.is_empty() {
            return None;
        }
        let id = self.active_ids[self.turn_index as usize].clone();
        self.turn_index = (self.turn_index + 1) % self.active_ids.len() as u64;
        Some(id)
    }

    pub fn skip_and_remove(&mut self, n: u64) -> Option<Id> {
        if self.active_ids.is_empty() || n == 0 {
            return None;
        }

        let mut last_removed = None;

        let mut idx = if self.turn_index == 0 {
            self.active_ids.len() - 1
        } else {
            (self.turn_index - 1) as usize
        };

        for _ in 0..n {
            if self.active_ids.is_empty() {
                break;
            }

            if idx >= self.active_ids.len() {
                idx = 0;
            }

            let removed = self.active_ids.remove(idx);
            last_removed = Some(removed.clone());

            if (self.turn_index as usize > idx)
                || (self.turn_index as usize == idx && self.turn_index > 0)
            {
                self.turn_index -= 1;
            }
        }

        let result_id = if self.active_ids.is_empty() {
            last_removed.expect("At least one player should have been removed")
        } else {
            let id = self.active_ids[self.turn_index as usize].clone();
            self.turn_index = (self.turn_index + 1) % self.active_ids.len() as u64;
            id
        };

        Some(result_id)
    }

    pub fn reset_turn_index(&mut self) {
        self.turn_index = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.active_ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.active_ids.len()
    }

    pub fn current(&self) -> Option<&Id> {
        self.active_ids.get(self.turn_index as usize)
    }

    pub fn all(&self) -> &Vec<Id> {
        &self.active_ids
    }

    pub fn get(&self, index: usize) -> Option<&Id> {
        self.active_ids.get(index)
    }

    pub fn peek_next(&self) -> Option<&Id> {
        if self.active_ids.is_empty() {
            return None;
        }
        let next_index = (self.turn_index + 1) % self.active_ids.len() as u64;
        self.active_ids.get(next_index as usize)
    }
    pub fn peek_prev(&self) -> Option<&Id> {
        if self.active_ids.is_empty() {
            return None;
        }

        let prev_index = if self.turn_index == 0 {
            self.active_ids.len() - 1
        } else {
            self.turn_index as usize - 1
        };

        self.active_ids.get(prev_index)
    }

    pub fn set_first_index(&mut self) {
        self.turn_index = self.first_index as u64;
    }

    pub fn remove_and_update_first_index(&mut self, id: &Id) {
        if let Some(pos) = self.active_ids.iter().position(|x| x == id) {
            self.active_ids.remove(pos);

            if self.first_index as usize > pos {
                self.first_index -= 1;
            } else if self.first_index as usize >= self.active_ids.len() {
                self.first_index = 0;
            }
        }
    }

    pub fn clear_all(&mut self) {
        self.active_ids.clear();
        self.turn_index = 0;
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BettingStage {
    pub turn: ActorId,
    pub last_active_time: Option<u64>,
    pub current_bet: u128,
    pub acted_players: Vec<ActorId>, // players who have placed a bet (Check or Call)
                                     // it's to keep track of when the lap ends
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Stage {
    PreFlop,
    WaitingTableCardsAfterPreFlop,
    Flop,
    WaitingTableCardsAfterFlop,
    Turn,
    WaitingTableCardsAfterTurn,
    River,
}

impl Stage {
    pub fn next(self) -> Option<Stage> {
        match self {
            Stage::PreFlop => Some(Stage::WaitingTableCardsAfterPreFlop),
            Stage::WaitingTableCardsAfterPreFlop => Some(Stage::Flop),
            Stage::Flop => Some(Stage::WaitingTableCardsAfterFlop),
            Stage::WaitingTableCardsAfterFlop => Some(Stage::Turn),
            Stage::Turn => Some(Stage::WaitingTableCardsAfterTurn),
            Stage::WaitingTableCardsAfterTurn => Some(Stage::River),
            Stage::River => None,
        }
    }
}

#[derive(Debug, Clone, Hash, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Suit {
    Spades,   // ♠
    Hearts,   // ♥
    Diamonds, // ♦
    Clubs,    // ♣
}

#[derive(Debug, Clone, Hash, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Card {
    pub value: u8, // 2–14 (where 11-J, 12-Q, 13-K, 14-A)
    pub suit: Suit,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        Card { suit, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard(Vec<u8>),
    Pair(u8, Vec<u8>),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, Vec<u8>),
    Straight(u8),
    Flush(Vec<u8>),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}

fn rank_hand(cards: Vec<Card>) -> HandRank {
    let mut values = Vec::new();
    let mut suits: HashMap<Suit, Vec<u8>> = HashMap::new();

    for card in &cards {
        values.push(card.value);
        suits.entry(card.suit.clone()).or_default().push(card.value);
    }

    values.sort_by(|a, b| b.cmp(a));
    let mut counts = HashMap::with_capacity(7);
    for &v in &values {
        *counts.entry(v).or_insert(0) += 1;
    }
    // Flush
    let flush = suits.iter().find(|(_, v)| v.len() >= 5).map(|(_, s)| {
        let mut s = s.clone();
        s.sort_by(|a, b| b.cmp(a));
        s.truncate(5);
        s
    });
    // Straight
    let mut unique = values.clone();
    unique.sort();
    unique.dedup();
    let mut straight = None;
    if unique.len() >= 5 {
        for w in unique.windows(5) {
            if w[0] + 1 == w[1] && w[1] + 1 == w[2] && w[2] + 1 == w[3] && w[3] + 1 == w[4] {
                straight = Some(w[4]);
                break;
            }
        }
        if unique.contains(&14)
            && unique.contains(&2)
            && unique.contains(&3)
            && unique.contains(&4)
            && unique.contains(&5)
        {
            straight = Some(5);
        }
    }

    // Straight Flush
    if let Some(s) = flush.clone() {
        let mut flush_unique = s.clone();
        flush_unique.sort();
        flush_unique.dedup();

        for w in flush_unique.windows(5) {
            if w[0] + 1 == w[1] && w[1] + 1 == w[2] && w[2] + 1 == w[3] && w[3] + 1 == w[4] {
                return HandRank::StraightFlush(w[4]);
            }
        }
    }

    let mut count_vec: Vec<_> = counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));

    match count_vec[..] {
        [(&a, &4), (&b, _)] => HandRank::FourOfAKind(a, b),
        [(&a, &3), (&b, &2), ..] => HandRank::FullHouse(a, b),
        [(&a, &3), ..] => {
            let kickers: Vec<u8> = values.iter().filter(|&&v| v != a).copied().collect();
            HandRank::ThreeOfAKind(a, kickers[..2].to_vec())
        }
        [(&a, &2), (&b, &2), (&c, _), ..] if a != b => HandRank::TwoPair(a.max(b), a.min(b), c),
        [(&a, &2), ..] => {
            let kickers: Vec<u8> = values.iter().filter(|&&v| v != a).copied().collect();
            HandRank::Pair(a, kickers[..3].to_vec())
        }
        _ => {
            if let Some(s) = flush {
                HandRank::Flush(s)
            } else if let Some(s) = straight {
                HandRank::Straight(s)
            } else {
                HandRank::HighCard(values)
            }
        }
    }
}

pub fn evaluate_round(
    hands: HashMap<ActorId, (Card, Card)>,
    table_cards: [Card; 5],
    bank: &HashMap<ActorId, u128>,
) -> Vec<(u128, Vec<ActorId>)> {
    let mut pots: Vec<(Vec<ActorId>, u128)> = Vec::new();
    let mut stakes: Vec<(ActorId, u128)> = bank.iter().map(|(id, amt)| (*id, *amt)).collect();
    stakes.sort_by_key(|&(_, amt)| amt);

    while !stakes.is_empty() {
        let (_, min_amt) = stakes[0];
        let mut pot = 0;
        let mut eligible = Vec::new();

        for (id, amount) in &mut stakes {
            if *amount >= min_amt {
                pot += min_amt;
                *amount -= min_amt;
                eligible.push(*id);
            } else {
                pot += *amount;
                *amount = 0;
                eligible.push(*id);
            }
        }

        pots.push((eligible.clone(), pot));
        stakes.retain(|&(_, amt)| amt > 0);
    }

    let mut rankings: HashMap<ActorId, HandRank> = HashMap::new();
    for (id, (c1, c2)) in &hands {
        let mut cards = vec![c1.clone(), c2.clone()];
        cards.extend_from_slice(&table_cards);
        rankings.insert(*id, rank_hand(cards));
    }

    let mut results: Vec<(u128, Vec<ActorId>)> = Vec::new();
    for (eligible, pot_amount) in pots {
        let mut ranked: Vec<_> = eligible
            .iter()
            .filter_map(|id| rankings.get(id).map(|r| (id, r)))
            .collect();

        ranked.sort_by(|a, b| b.1.cmp(a.1)); // strongest hand first

        if let Some(best_rank) = ranked.clone().first().map(|(_, rank)| rank) {
            let winners: Vec<ActorId> = ranked
                .into_iter()
                .filter(|(_, rank)| rank == best_rank)
                .map(|(id, _)| *id)
                .collect();

            results.push((pot_amount, winners));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_pots_eq(actual: Vec<(u128, Vec<ActorId>)>, expected: Vec<(u128, Vec<ActorId>)>) {
        assert_eq!(actual.len(), expected.len(), "Number of pots differ");
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert_eq!(a.0, e.0, "Pot amounts differ");
            let mut actual_winners = a.1.clone();
            let mut expected_winners = e.1.clone();
            actual_winners.sort();
            expected_winners.sort();
            assert_eq!(actual_winners, expected_winners, "Pot winners differ");
        }
    }

    #[test]
    fn test_high_card() {
        // player 2 should win whole pot
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 6), Card::new(Suit::Spades, 12)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 9), Card::new(Suit::Hearts, 14)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Diamonds, 11), Card::new(Suit::Spades, 13)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 5),
            Card::new(Suit::Hearts, 10),
            Card::new(Suit::Clubs, 7),
            Card::new(Suit::Diamonds, 4),
            Card::new(Suit::Diamonds, 2),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);
        bank.insert(3.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(300, vec![2.into()])]);
    }

    #[test]
    fn test_straight_flush() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 6), Card::new(Suit::Hearts, 8)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 9), Card::new(Suit::Spades, 4)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Spades, 7), Card::new(Suit::Spades, 3)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 5),
            Card::new(Suit::Hearts, 9),
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Diamonds, 4),
            Card::new(Suit::Diamonds, 10),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);
        bank.insert(3.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(300, vec![1.into()])]);
    }

    #[test]
    fn test_straight_and_flush() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 6), Card::new(Suit::Hearts, 11)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 9), Card::new(Suit::Spades, 11)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Spades, 7), Card::new(Suit::Spades, 3)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 5),
            Card::new(Suit::Hearts, 8),
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Diamonds, 4),
            Card::new(Suit::Diamonds, 10),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);
        bank.insert(3.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(300, vec![1.into()])]);
    }

    #[test]
    fn test_pair_kicker() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 10), Card::new(Suit::Clubs, 14)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 10), Card::new(Suit::Diamonds, 3)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 4),
            Card::new(Suit::Spades, 10),
            Card::new(Suit::Hearts, 13),
            Card::new(Suit::Diamonds, 13),
            Card::new(Suit::Clubs, 2),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn test_wheel_straight_vs_high_straight() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 2), Card::new(Suit::Clubs, 3)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 8), Card::new(Suit::Diamonds, 9)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 4),
            Card::new(Suit::Spades, 5),
            Card::new(Suit::Hearts, 14),
            Card::new(Suit::Diamonds, 6),
            Card::new(Suit::Clubs, 7),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![2.into()])]);
    }

    #[test]
    fn test_three_of_a_kind_vs_two_pair() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 7), Card::new(Suit::Clubs, 7)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 10), Card::new(Suit::Diamonds, 10)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Spades, 6),
            Card::new(Suit::Hearts, 6),
            Card::new(Suit::Diamonds, 2),
            Card::new(Suit::Clubs, 3),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn test_full_house_beats_flush() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 5), Card::new(Suit::Hearts, 2)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 6), Card::new(Suit::Diamonds, 6)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 6),
            Card::new(Suit::Clubs, 7),
            Card::new(Suit::Hearts, 8),
            Card::new(Suit::Hearts, 9),
            Card::new(Suit::Clubs, 9),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![2.into()])]);
    }

    #[test]
    fn test_side_pot_split() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Diamonds, 14)),
        ); // AA
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 13), Card::new(Suit::Clubs, 13)),
        ); // KK
        hands.insert(
            3.into(),
            (Card::new(Suit::Spades, 2), Card::new(Suit::Clubs, 3)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 10),
            Card::new(Suit::Diamonds, 9),
            Card::new(Suit::Clubs, 4),
            Card::new(Suit::Spades, 7),
            Card::new(Suit::Diamonds, 6),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 200);
        bank.insert(3.into(), 200);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(
            pots,
            vec![
                (300, vec![1.into()]), // main pot
                (200, vec![2.into()]), // side pot
            ],
        );
    }

    #[test]
    fn test_split_pot_same_hand() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 10), Card::new(Suit::Clubs, 9)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 10), Card::new(Suit::Diamonds, 9)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Spades, 2), Card::new(Suit::Hearts, 3)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 8),
            Card::new(Suit::Diamonds, 7),
            Card::new(Suit::Clubs, 6),
            Card::new(Suit::Spades, 4),
            Card::new(Suit::Hearts, 2),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 150);
        bank.insert(2.into(), 150);
        bank.insert(3.into(), 150);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(450, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn test_pots_1() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 6), Card::new(Suit::Hearts, 8)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 13), Card::new(Suit::Hearts, 3)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Hearts, 13), Card::new(Suit::Diamonds, 8)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Clubs, 5),
            Card::new(Suit::Diamonds, 14),
            Card::new(Suit::Spades, 13),
            Card::new(Suit::Spades, 9),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 500);
        bank.insert(3.into(), 500);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(300, vec![1.into()]), (800, vec![3.into()])]);
    }

    #[test]
    fn test_pots_2() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 6), Card::new(Suit::Hearts, 8)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 13), Card::new(Suit::Hearts, 3)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Hearts, 13), Card::new(Suit::Diamonds, 3)),
        );

        let table_cards = [
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Clubs, 5),
            Card::new(Suit::Diamonds, 14),
            Card::new(Suit::Spades, 13),
            Card::new(Suit::Spades, 9),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 500);
        bank.insert(3.into(), 500);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(
            pots,
            vec![(300, vec![1.into()]), (800, vec![2.into(), 3.into()])],
        );
    }
}
