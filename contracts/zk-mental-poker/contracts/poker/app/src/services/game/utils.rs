use core::fmt::Debug;
use sails_rs::collections::HashMap;
use sails_rs::prelude::*;
use sails_rs::{ActorId, Vec};

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
        if self.active_ids.is_empty() {
            self.first_index = 0;
            return;
        }
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

    pub fn skip_and_remove(&mut self, mut n: u64) -> Option<Id> {
        if self.active_ids.is_empty() || n == 0 {
            return None;
        }

        let mut last_removed: Option<Id> = None;

        let mut current_idx = if self.turn_index == 0 {
            self.active_ids.len() - 1
        } else {
            (self.turn_index - 1) as usize
        };

        while n > 0 && !self.active_ids.is_empty() {
            if current_idx >= self.active_ids.len() {
                current_idx = 0;
            }

            last_removed = Some(self.active_ids.remove(current_idx));

            n -= 1;
        }

        if self.active_ids.is_empty() {
            self.turn_index = 0;
            return last_removed;
        }

        if current_idx >= self.active_ids.len() {
            current_idx = 0;
        }

        let result_id = self.active_ids[current_idx].clone();
        self.turn_index = ((current_idx as u64) + 1) % self.active_ids.len() as u64;

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
        if self.active_ids.is_empty() {
            self.turn_index = 0;
            self.first_index = 0;
            return;
        }

        if self.first_index as usize >= self.active_ids.len() {
            self.first_index = 0;
        }

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
        self.first_index = 0;
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

fn highest_straight_high(values: &[u8]) -> Option<u8> {
    // Finds the BEST (according to the senior card) straight in the set of values (ace = 14)
    let mut u = values.to_vec();
    u.sort();
    u.dedup();

    // standard streets — search from the end (highest)
    for w in u.windows(5).rev() {
        if w[0] + 1 == w[1] && w[1] + 1 == w[2] && w[2] + 1 == w[3] && w[3] + 1 == w[4] {
            return Some(w[4]);
        }
    }

    // wheel A-2-3-4-5
    if u.contains(&14) && u.contains(&2) && u.contains(&3) && u.contains(&4) && u.contains(&5) {
        return Some(5);
    }

    None
}

fn rank_hand(cards: Vec<Card>) -> HandRank {
    let mut values: Vec<u8> = Vec::new();
    let mut suits: HashMap<Suit, Vec<u8>> = HashMap::new();

    for card in &cards {
        values.push(card.value);
        suits.entry(card.suit.clone()).or_default().push(card.value);
    }

    // ----- Straight Flush -----
    let mut best_sf: Option<u8> = None;
    for vs in suits.values() {
        if vs.len() >= 5
            && let Some(h) = highest_straight_high(vs)
        {
            best_sf = Some(best_sf.map_or(h, |cur| cur.max(h)));
        }
    }
    if let Some(h) = best_sf {
        return HandRank::StraightFlush(h);
    }

    values.sort_by(|a, b| b.cmp(a));
    let mut counts: HashMap<u8, u8> = HashMap::with_capacity(7);
    for &v in &values {
        *counts.entry(v).or_insert(0) += 1;
    }
    let mut count_vec: Vec<_> = counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));

    // ----- Four of a Kind / Full House -----
    match count_vec[..] {
        [(&a, &4), (&b, _), ..] => return HandRank::FourOfAKind(a, b),
        [(&a, &3), (&b, &3), ..] => return HandRank::FullHouse(a.max(b), a.min(b)),
        [(&a, &3), (&b, &2), ..] => return HandRank::FullHouse(a, b),
        _ => {}
    }

    // ----- Flush -----
    let mut best_flush: Option<Vec<u8>> = None;
    for vs in suits.values() {
        if vs.len() >= 5 {
            let mut t = vs.clone();
            t.sort_by(|a, b| b.cmp(a));
            t.truncate(5);
            if best_flush.as_ref().is_none_or(|b| &t > b) {
                best_flush = Some(t);
            }
        }
    }
    if let Some(s) = best_flush {
        return HandRank::Flush(s);
    }

    // ----- Straight -----
    if let Some(h) = highest_straight_high(&values) {
        return HandRank::Straight(h);
    }

    // ----- Trips / TwoPair / Pair -----
    match count_vec[..] {
        [(&a, &3), ..] => {
            let kickers: Vec<u8> = values.iter().copied().filter(|&v| v != a).take(2).collect();
            return HandRank::ThreeOfAKind(a, kickers);
        }
        [(&a, &2), (&b, &2), ..] if a != b => {
            let kicker = count_vec
                .iter()
                .filter_map(|&(&val, &cnt)| {
                    if val != a && val != b && cnt >= 1 {
                        Some(val)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            return HandRank::TwoPair(a.max(b), a.min(b), kicker);
        }
        [(&a, &2), ..] => {
            let kickers: Vec<u8> = values.iter().copied().filter(|&v| v != a).take(3).collect();
            return HandRank::Pair(a, kickers);
        }
        _ => {}
    }

    // ----- High card -----
    let mut top5 = values.clone();
    top5.truncate(5);
    HandRank::HighCard(top5)
}

pub fn evaluate_round(
    hands: HashMap<ActorId, (Card, Card)>,
    table_cards: [Card; 5],
    bank: &HashMap<ActorId, u128>,
) -> Vec<(u128, Vec<ActorId>)> {
    let mut pots: Vec<(Vec<ActorId>, u128)> = Vec::new();
    let mut stakes: Vec<(ActorId, u128)> = bank
        .iter()
        .filter_map(|(id, &amt)| (amt > 0).then_some((*id, amt)))
        .collect();

    stakes.sort_unstable_by_key(|&(_, amt)| amt);

    while !stakes.is_empty() {
        let min_amt = stakes[0].1;
        if min_amt == 0 {
            stakes.retain(|&(_, amt)| amt > 0);
            continue;
        }
        let mut pot = 0;
        let mut eligible = Vec::new();

        for (id, amount) in &mut stakes {
            let take = min_amt.min(*amount);
            pot += take;
            *amount -= take;
            eligible.push(*id);
        }

        if pot > 0 {
            pots.push((eligible.clone(), pot));
        }

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
        let mut ranked: Vec<(&ActorId, &HandRank)> = eligible
            .iter()
            .filter_map(|id| rankings.get(id).map(|r| (id, r)))
            .collect();

        if ranked.is_empty() {
            continue;
        }

        ranked.sort_by(|a, b| b.1.cmp(a.1));

        if let Some(best_rank) = ranked.first().map(|(_, r)| (*r).clone()) {
            let winners: Vec<ActorId> = ranked
                .into_iter()
                .take_while(|(_, r)| *r == &best_rank)
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

    #[test]
    fn test_split_same_board_straight() {
        let table_cards = [
            Card::new(Suit::Hearts, 5),
            Card::new(Suit::Clubs, 6),
            Card::new(Suit::Diamonds, 7),
            Card::new(Suit::Spades, 8),
            Card::new(Suit::Hearts, 9),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Clubs, 13), Card::new(Suit::Spades, 14)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 4), Card::new(Suit::Clubs, 2)),
        );

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn test_straight_flush_not_only_from_top5_suited() {
        // A♥ 9♥ 5♥ 4♥
        let table_cards = [
            Card::new(Suit::Hearts, 14),
            Card::new(Suit::Hearts, 9),
            Card::new(Suit::Hearts, 5),
            Card::new(Suit::Hearts, 4),
            Card::new(Suit::Clubs, 7),
        ];

        let mut hands = HashMap::new();
        // player 1 get 3♥ 2♥ => A♥,5♥,4♥,3♥,2♥
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 3), Card::new(Suit::Hearts, 2)),
        );
        // player 2 without ♥
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 14), Card::new(Suit::Diamonds, 12)),
        );

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn test_high_card_uses_only_top5() {
        let table_cards = [
            Card::new(Suit::Spades, 13),   // K
            Card::new(Suit::Diamonds, 12), // Q
            Card::new(Suit::Clubs, 9),
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Clubs, 4),
        ];

        let mut hands = HashMap::new();
        // Both get A-high
        hands.insert(
            1.into(),
            (Card::new(Suit::Clubs, 14), Card::new(Suit::Diamonds, 2)),
        ); // A,2
        hands.insert(
            2.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Spades, 3)),
        ); // A,3

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn test_wheel_does_not_override_higher_straight() {
        let table_cards = [
            Card::new(Suit::Hearts, 14), // A
            Card::new(Suit::Diamonds, 2),
            Card::new(Suit::Clubs, 3),
            Card::new(Suit::Spades, 4),
            Card::new(Suit::Clubs, 13), // K
        ];

        let mut hands = HashMap::new();
        // Player 1: 5 и 6 => Straight 2-6
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 5), Card::new(Suit::Diamonds, 6)),
        );
        // Player 2: only 5 => wheel (5-high)
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 5), Card::new(Suit::Clubs, 9)),
        );

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn test_full_house_from_double_trips() {
        // 7 7 7 9 9
        let table = [
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Clubs, 7),
            Card::new(Suit::Diamonds, 7),
            Card::new(Suit::Spades, 9),
            Card::new(Suit::Hearts, 9),
        ];

        // Player 1: 9 => 9-9-9 и 7-7-7 => FullHouse(9,7)
        // Player 2: A A => FullHouse(7,14))
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Clubs, 9), Card::new(Suit::Spades, 2)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Clubs, 14), Card::new(Suit::Diamonds, 14)),
        );

        let mut bank = HashMap::new();
        bank.insert(1.into(), 200);
        bank.insert(2.into(), 200);

        let pots = evaluate_round(hands, table, &bank);
        // (FullHouse(9,7) > FullHouse(7,14))
        assert_pots_eq(pots, vec![(400, vec![1.into()])]);
    }

    #[test]
    fn test_zero_bank_entries_are_ignored() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Clubs, 2)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 13), Card::new(Suit::Diamonds, 3)),
        );

        let table = [
            Card::new(Suit::Clubs, 10),
            Card::new(Suit::Clubs, 9),
            Card::new(Suit::Hearts, 8),
            Card::new(Suit::Spades, 7),
            Card::new(Suit::Diamonds, 6),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 0); // zero contribution to waste
        bank.insert(2.into(), 200); // real contribution

        let pots = evaluate_round(hands, table, &bank);
        assert_pots_eq(pots, vec![(200, vec![2.into()])]);
    }

    #[test]
    fn test_board_quads_kicker_matters() {
        // 9 9 9 9 2
        let table = [
            Card::new(Suit::Hearts, 9),
            Card::new(Suit::Clubs, 9),
            Card::new(Suit::Diamonds, 9),
            Card::new(Suit::Spades, 9),
            Card::new(Suit::Hearts, 2),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Clubs, 14), Card::new(Suit::Spades, 3)),
        ); // A-kicker
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 13), Card::new(Suit::Clubs, 3)),
        ); // K-kicker

        let mut bank = HashMap::new();
        bank.insert(1.into(), 150);
        bank.insert(2.into(), 150);

        let pots = evaluate_round(hands, table, &bank);
        assert_pots_eq(pots, vec![(300, vec![1.into()])]);
    }

    #[test]
    fn test_main_and_two_side_pots_with_ties() {
        // A=100, B=200, C=350
        let mut bank = HashMap::new();
        bank.insert(1.into(), 100); // A
        bank.insert(2.into(), 200); // B
        bank.insert(3.into(), 350); // C

        // Let's select so that:
        // - In the main pot, A and B split, C loses
        // - In the first side pot (B vs C), C wins
        // - In the second side pot (only C) — obviously C
        let mut hands = HashMap::new();
        // A: Straight 10-A
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Clubs, 10)),
        );
        // B: also a straight 10-A
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 14), Card::new(Suit::Diamonds, 10)),
        );
        // C: only a pair (loses to the main pot), but a strong hand for the side pot
        hands.insert(
            3.into(),
            (Card::new(Suit::Hearts, 13), Card::new(Suit::Spades, 13)),
        );

        let table = [
            Card::new(Suit::Clubs, 11),
            Card::new(Suit::Diamonds, 12),
            Card::new(Suit::Spades, 13),
            Card::new(Suit::Hearts, 9),
            Card::new(Suit::Clubs, 8),
        ];

        let pots = evaluate_round(hands, table, &bank);

        // We expect:
        // main: 100*3 = 300 → divide A and B
        // side1: (200-100)*2 = 200 → B vs C, C wins
        // side2: (350-200) = 150 → only C
        assert_pots_eq(
            pots,
            vec![
                (300, vec![1.into(), 2.into()]),
                (200, vec![2.into()]),
                (150, vec![3.into()]),
            ],
        );
    }

    #[test]
    fn flush_beats_two_pair_even_if_pair_present() {
        let table = [
            Card::new(Suit::Hearts, 2),
            Card::new(Suit::Hearts, 9),
            Card::new(Suit::Hearts, 5),
            Card::new(Suit::Diamonds, 7),
            Card::new(Suit::Clubs, 7),
        ];

        let mut hands = HashMap::new();
        // A: (A-high flush)
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Hearts, 3)),
        ); // A♥,3♥
        // B: pair K7
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 13), Card::new(Suit::Diamonds, 13)),
        ); // K♠,K♦

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table, &bank);
        assert_eq!(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn pair_vs_pair_kicker_ace_wins() {
        let table_cards = [
            Card::new(Suit::Hearts, 12), // Q
            Card::new(Suit::Clubs, 12),  // Q
            Card::new(Suit::Clubs, 8),
            Card::new(Suit::Diamonds, 5),
            Card::new(Suit::Spades, 3),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Clubs, 14), Card::new(Suit::Diamonds, 9)),
        ); // A,9
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 13), Card::new(Suit::Diamonds, 11)),
        ); // K,J

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn pair_split_all_kickers_equal() {
        let table_cards = [
            Card::new(Suit::Hearts, 12),   // Q
            Card::new(Suit::Clubs, 12),    // Q
            Card::new(Suit::Diamonds, 10), // 10
            Card::new(Suit::Clubs, 9),     // 9
            Card::new(Suit::Spades, 8),    // 8
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Clubs, 14), Card::new(Suit::Diamonds, 7)),
        ); // A,7
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 14), Card::new(Suit::Clubs, 7)),
        ); // A,7

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn two_pair_beats_pair() {
        let table_cards = [
            Card::new(Suit::Clubs, 13),    // K
            Card::new(Suit::Diamonds, 13), // K
            Card::new(Suit::Hearts, 9),    // 9
            Card::new(Suit::Spades, 5),
            Card::new(Suit::Clubs, 2),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 9), Card::new(Suit::Diamonds, 4)),
        ); // 9,4
        hands.insert(
            2.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Hearts, 3)),
        ); // A,3

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn two_pair_tie_kicker_decides() {
        let table_cards = [
            Card::new(Suit::Clubs, 13),    // K
            Card::new(Suit::Diamonds, 13), // K
            Card::new(Suit::Hearts, 9),    // 9
            Card::new(Suit::Spades, 5),
            Card::new(Suit::Clubs, 2),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 9), Card::new(Suit::Diamonds, 14)),
        ); // 9,A
        hands.insert(
            2.into(),
            (Card::new(Suit::Clubs, 9), Card::new(Suit::Clubs, 12)),
        ); // 9,Q

        let mut bank = HashMap::new();
        bank.insert(1.into(), 200);
        bank.insert(2.into(), 200);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(400, vec![1.into()])]);
    }

    #[test]
    fn two_pair_split_same_kicker() {
        let table_cards = [
            Card::new(Suit::Clubs, 13),    // K
            Card::new(Suit::Diamonds, 13), // K
            Card::new(Suit::Hearts, 9),    // 9
            Card::new(Suit::Spades, 5),
            Card::new(Suit::Clubs, 14), // A
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 9), Card::new(Suit::Diamonds, 12)),
        ); // 9,Q
        hands.insert(
            2.into(),
            (Card::new(Suit::Clubs, 9), Card::new(Suit::Clubs, 3)),
        ); // 9,3

        let mut bank = HashMap::new();
        bank.insert(1.into(), 150);
        bank.insert(2.into(), 150);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(300, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn trips_tie_kickers_decide() {
        let table = [
            Card::new(Suit::Spades, 7),
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Diamonds, 7),
            Card::new(Suit::Clubs, 13), // K
            Card::new(Suit::Clubs, 12), // Q
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Spades, 14), Card::new(Suit::Diamonds, 10)),
        ); // A,10
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 14), Card::new(Suit::Clubs, 9)),
        ); // A,9

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn trips_kickers_hand_decides() {
        let table = [
            Card::new(Suit::Spades, 7),
            Card::new(Suit::Hearts, 7),
            Card::new(Suit::Diamonds, 7),
            Card::new(Suit::Clubs, 8),
            Card::new(Suit::Clubs, 2),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Spades, 14), Card::new(Suit::Diamonds, 10)),
        ); // A,10
        hands.insert(
            2.into(),
            (Card::new(Suit::Diamonds, 14), Card::new(Suit::Clubs, 9)),
        ); // A,9

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table, &bank);

        assert_pots_eq(pots, vec![(200, vec![1.into()])]);
    }

    #[test]
    fn trips_beats_two_pair_general() {
        let table = [
            Card::new(Suit::Hearts, 8),
            Card::new(Suit::Spades, 6),
            Card::new(Suit::Clubs, 13), // K
            Card::new(Suit::Diamonds, 2),
            Card::new(Suit::Clubs, 3),
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 8), Card::new(Suit::Hearts, 8)),
        ); // 8,8 → Trips
        hands.insert(
            2.into(),
            (Card::new(Suit::Spades, 13), Card::new(Suit::Diamonds, 8)),
        ); // K,8 → TwoPair

        let mut bank = HashMap::new();
        bank.insert(1.into(), 120);
        bank.insert(2.into(), 120);

        let pots = evaluate_round(hands, table, &bank);
        assert_pots_eq(pots, vec![(240, vec![1.into()])]);
    }

    #[test]
    fn board_broadway_straight_split() {
        let table_cards = [
            Card::new(Suit::Clubs, 10),    // T
            Card::new(Suit::Diamonds, 11), // J
            Card::new(Suit::Hearts, 12),   // Q
            Card::new(Suit::Spades, 13),   // K
            Card::new(Suit::Clubs, 14),    // A
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 14), Card::new(Suit::Diamonds, 14)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Clubs, 2), Card::new(Suit::Spades, 2)),
        );

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![1.into(), 2.into()])]);
    }

    #[test]
    fn straight_beats_trips() {
        let table_cards = [
            Card::new(Suit::Clubs, 2),
            Card::new(Suit::Diamonds, 3),
            Card::new(Suit::Hearts, 4),
            Card::new(Suit::Spades, 9),
            Card::new(Suit::Diamonds, 13), // K
        ];

        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Hearts, 9), Card::new(Suit::Diamonds, 9)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Clubs, 5), Card::new(Suit::Diamonds, 6)),
        );

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 100);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_pots_eq(pots, vec![(200, vec![2.into()])]);
    }

    fn tm_with(ids: &[u8], turn_index: u64) -> TurnManager<u8> {
        let mut tm: TurnManager<u8> = TurnManager::new();
        for &id in ids {
            tm.add(id);
        }
        tm.turn_index = turn_index;
        tm
    }

    #[test]
    fn skip_and_remove_one_basic() {
        let mut tm = tm_with(&[1, 2, 3, 4], 1);

        let next = tm.skip_and_remove(1).expect("must have next");

        assert_eq!(next, 2);
        assert_eq!(tm.all(), &vec![2, 3, 4]);
        assert_eq!(tm.turn_index, 1);
    }

    #[test]
    fn skip_and_remove_two_in_row() {
        let mut tm = tm_with(&[1, 2, 3, 4], 1);
        let next = tm.skip_and_remove(2).expect("must have next");

        assert_eq!(next, 3);
        assert_eq!(tm.all(), &vec![3, 4]);
        assert_eq!(tm.turn_index, 1);
    }

    #[test]
    fn skip_and_remove_three_from_four() {
        let mut tm = tm_with(&[1, 2, 3, 4], 1);

        let next = tm.skip_and_remove(3).expect("must have next");

        assert_eq!(next, 4);
        assert_eq!(tm.all(), &vec![4]);
        assert_eq!(tm.turn_index, 0);
    }

    #[test]
    fn skip_and_remove_more_than_len() {
        let mut tm = tm_with(&[1, 2, 3], 1);
        let next = tm.skip_and_remove(10).expect("must have next");

        assert_eq!(next, 3);
        assert!(tm.is_empty());
    }

    #[test]
    fn skip_and_remove_with_single_player() {
        let mut tm = tm_with(&[7], 0);

        let next = tm.skip_and_remove(1).expect("must have next");

        assert_eq!(next, 7);
        assert!(tm.is_empty());
    }

    #[test]
    fn skip_and_remove_equivalent_to_repeated_single_skips() {
        let mut tm_bulk = tm_with(&[1, 2, 3, 4], 1);
        let mut tm_step = tm_bulk.clone();

        let bulk_next = tm_bulk.skip_and_remove(3);
        let mut step_next = None;
        for _ in 0..3 {
            step_next = tm_step.skip_and_remove(1);
        }

        assert_eq!(bulk_next, step_next);
        assert_eq!(tm_bulk.all(), tm_step.all());
        assert_eq!(tm_bulk.turn_index, tm_step.turn_index);
    }

    #[test]
    fn bug_case_flush_vs_pair_sidepots() {
        let mut hands = HashMap::new();
        hands.insert(
            1.into(),
            (Card::new(Suit::Diamonds, 11), Card::new(Suit::Diamonds, 12)),
        );
        hands.insert(
            2.into(),
            (Card::new(Suit::Clubs, 9), Card::new(Suit::Hearts, 8)),
        );
        hands.insert(
            3.into(),
            (Card::new(Suit::Diamonds, 4), Card::new(Suit::Diamonds, 10)),
        );

        let table_cards = [
            Card::new(Suit::Diamonds, 6),
            Card::new(Suit::Diamonds, 9),
            Card::new(Suit::Spades, 4),
            Card::new(Suit::Hearts, 12),
            Card::new(Suit::Diamonds, 5),
        ];

        let mut bank = HashMap::new();
        bank.insert(1.into(), 100);
        bank.insert(2.into(), 500);
        bank.insert(3.into(), 150);

        let pots = evaluate_round(hands, table_cards, &bank);
        assert_eq!(
            pots,
            vec![
                (300, vec![1.into()]),
                (100, vec![3.into()]),
                (350, vec![2.into()]),
            ]
        );
    }
}
