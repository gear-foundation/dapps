use crate::services::game::{Card, EncryptedCard, Suit, VerificationVariables, ZkPublicKey};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ed_on_bls12_381_bandersnatch::{EdwardsAffine, EdwardsProjective, Fq, Fr};
use ark_ff::{BigInteger, One, PrimeField, Zero};
use sails_rs::{collections::HashMap, prelude::*};

pub fn deserialize_bandersnatch_coords(coords: &[Vec<u8>; 3]) -> EdwardsProjective {
    let x = Fq::from_le_bytes_mod_order(&coords[0]);
    let y = Fq::from_le_bytes_mod_order(&coords[1]);
    let z = Fq::from_le_bytes_mod_order(&coords[2]);
    let t = x * y;

    EdwardsProjective::new_unchecked(x, y, t, z)
        .into_affine()
        .into()
}

pub fn serialize_bandersnatch_coords(point: &EdwardsProjective) -> [Vec<u8>; 3] {
    [
        point.x.into_bigint().to_bytes_le(),
        point.y.into_bigint().to_bytes_le(),
        point.z.into_bigint().to_bytes_le(),
    ]
}

fn deserialize_public_key(pk: &ZkPublicKey) -> EdwardsProjective {
    let x = Fq::from_le_bytes_mod_order(&pk.x);
    let y = Fq::from_le_bytes_mod_order(&pk.y);
    let z = Fq::from_le_bytes_mod_order(&pk.z);
    let t = x * y;

    EdwardsProjective::new(x, y, t, z)
}

fn serialize_public_key(point: &EdwardsProjective) -> ZkPublicKey {
    let x = point
        .x
        .into_bigint()
        .to_bytes_le()
        .try_into()
        .expect("x not 32 bytes");
    let y = point
        .y
        .into_bigint()
        .to_bytes_le()
        .try_into()
        .expect("y not 32 bytes");
    let z = point
        .z
        .into_bigint()
        .to_bytes_le()
        .try_into()
        .expect("z not 32 bytes");

    ZkPublicKey { x, y, z }
}

pub fn calculate_agg_pub_key(pk1: &ZkPublicKey, pk2: &ZkPublicKey) -> ZkPublicKey {
    let point1 = deserialize_public_key(pk1);
    let point2 = deserialize_public_key(pk2);

    let result = point1 + point2;
    serialize_public_key(&result)
}

pub fn substract_agg_pub_key(pk1: &ZkPublicKey, pk2: &ZkPublicKey) -> ZkPublicKey {
    let point1 = deserialize_public_key(pk1);
    let point2 = deserialize_public_key(pk2);

    let result = point1 - point2;
    serialize_public_key(&result)
}

pub fn compare_public_keys(pk1: &ZkPublicKey, pk2: &ZkPublicKey) -> bool {
    let x1 = Fq::from_le_bytes_mod_order(&pk1.x);
    let y1 = Fq::from_le_bytes_mod_order(&pk1.y);
    let z1 = Fq::from_le_bytes_mod_order(&pk1.z);

    let x2 = Fq::from_le_bytes_mod_order(&pk2.x);
    let y2 = Fq::from_le_bytes_mod_order(&pk2.y);
    let z2 = Fq::from_le_bytes_mod_order(&pk2.z);

    (x1 * z2 == x2 * z1) && (y1 * z2 == y2 * z1)
}

pub fn init_deck_and_card_map() -> (Vec<EdwardsProjective>, HashMap<EdwardsProjective, Card>) {
    let mut encrypted_deck: Vec<EdwardsProjective> = Vec::with_capacity(52);

    let num_cards = 52;
    let base_affine = EdwardsAffine::generator();
    let base_point: EdwardsProjective = base_affine.into();

    for i in 1..=num_cards {
        let scalar = Fr::from(i as u64);
        let point = base_point * scalar;

        encrypted_deck.push(point);
    }

    let card_map = build_card_map(encrypted_deck.clone());

    (encrypted_deck, card_map)
}

pub fn build_card_map(deck: Vec<EdwardsProjective>) -> HashMap<EdwardsProjective, Card> {
    let mut card_map = HashMap::new();

    let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    let values = 2..=14;

    let mut index = 0;
    for suit in &suits {
        for value in values.clone() {
            card_map.insert(deck[index], Card::new(suit.clone(), value));

            index += 1;
        }
    }

    card_map
}

pub fn compare_points(p1: &[Vec<u8>; 3], p2: &[Vec<u8>; 3]) -> bool {
    let x1 = Fq::from_le_bytes_mod_order(&p1[0]);
    let y1 = Fq::from_le_bytes_mod_order(&p1[1]);
    let z1 = Fq::from_le_bytes_mod_order(&p1[2]);

    let x2 = Fq::from_le_bytes_mod_order(&p2[0]);
    let y2 = Fq::from_le_bytes_mod_order(&p2[1]);
    let z2 = Fq::from_le_bytes_mod_order(&p2[2]);

    (x1 * z2 == x2 * z1) && (y1 * z2 == y2 * z1)
}

pub fn compare_projective_and_coords(
    projective: &EdwardsProjective,
    coords: &[Vec<u8>; 3],
) -> bool {
    let x2 = Fq::from_le_bytes_mod_order(&coords[0]);
    let y2 = Fq::from_le_bytes_mod_order(&coords[1]);
    let z2 = Fq::from_le_bytes_mod_order(&coords[2]);

    points_equal_coords(&projective.x, &projective.y, &projective.z, &x2, &y2, &z2)
}

#[inline(always)]
fn points_equal_coords(x1: &Fq, y1: &Fq, z1: &Fq, x2: &Fq, y2: &Fq, z2: &Fq) -> bool {
    (x1 * z2 == x2 * z1) && (y1 * z2 == y2 * z1)
}

pub fn find_card_by_point(
    card_map: &HashMap<EdwardsProjective, Card>,
    point: &EdwardsProjective,
) -> Option<Card> {
    card_map.iter().find_map(|(p, card)| {
        if (point.x * p.z == p.x * point.z) && (point.y * p.z == p.y * point.z) {
            Some(card.clone())
        } else {
            None
        }
    })
}

pub fn decrypt_point(
    card_map: &HashMap<EdwardsProjective, Card>,
    encrypted_point: &EncryptedCard,
    partial_decryptions: Vec<[Vec<u8>; 3]>,
) -> Option<Card> {
    let mut sum = deserialize_bandersnatch_coords(&partial_decryptions[0]);
    for partial in partial_decryptions.iter().skip(1) {
        sum += deserialize_bandersnatch_coords(partial);
    }
    let c1_point = deserialize_bandersnatch_coords(&encrypted_point.c1);
    let decrypted_point = c1_point + sum;
    find_card_by_point(card_map, &decrypted_point)
}

pub fn verify_cards(
    partially_decrypted: &[EncryptedCard; 2],
    instances: Vec<(Card, VerificationVariables)>,
    card_map: &HashMap<EdwardsProjective, Card>,
) {
    if instances.len() != 2 {
        panic!("Expected 2 cards with proofs");
    }

    let c01 = partially_decrypted[0].clone().c0;
    let c02 = partially_decrypted[1].clone().c0;

    for (declared_card, instance) in instances.into_iter() {
        let (c0, c1_part_coords) = parse_partial_decryption_inputs(&instance.public_input);

        let encrypted = if compare_points(&c0, &c01) {
            &partially_decrypted[0]
        } else if compare_points(&c0, &c02) {
            &partially_decrypted[1]
        } else {
            panic!("Card not found")
        };

        let c1_point = deserialize_bandersnatch_coords(&encrypted.c1);
        let c1_part = deserialize_bandersnatch_coords(&c1_part_coords);

        let decrypted_point = c1_point + c1_part;

        let Some(expected_card) = find_card_by_point(card_map, &decrypted_point) else {
            panic!("Decrypted point not found in card map");
        };
        if declared_card != expected_card {
            panic!("Declared card does not match actual decrypted card");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ark_ec::CurveGroup;
//     use ark_ed_on_bls12_381_bandersnatch::{EdwardsProjective, Fq};
//     use ark_ff::UniformRand;
//     use ark_std::Zero;
//     use num_bigint::BigUint;
//     use rand::rngs::OsRng;
//     use serde::Deserialize;

//     const CARD_MAP_JSON: &str = include_str!("../../test_data/card_map.json");

//     #[cfg(test)]
//     extern crate std;
//     #[cfg(test)]
//     use std::println;

//     #[derive(Debug, Deserialize)]
//     struct JsonCard {
//         suit: String,
//         rank: String,
//         point: JsonPoint,
//     }

//     #[derive(Debug, Deserialize)]
//     struct JsonPoint {
//         X: String,
//         Y: String,
//         Z: String,
//     }

//     fn decimal_str_to_bytes_32_be(s: &str) -> [u8; 32] {
//         let n = BigUint::parse_bytes(s.as_bytes(), 10).expect("Invalid decimal string");
//         let mut b = n.to_bytes_be();
//         if b.len() > 32 {
//             panic!("Value too large for 32 bytes");
//         }
//         let mut buf = [0u8; 32];
//         buf[32 - b.len()..].copy_from_slice(&b);
//         buf
//     }

//     fn key_gen() -> (Fr, EdwardsProjective) {
//         let mut rng = ark_std::test_rng();
//         let sk = Fr::rand(&mut rng);

//         let base_affine = EdwardsAffine::generator();
//         let base_point: EdwardsProjective = base_affine.into();

//         let pk = base_point * sk;

//         (sk, pk)
//     }

//     fn elgamal_encrypt(
//         pk: EdwardsProjective,
//         i_c0: EdwardsProjective,
//         i_c1: EdwardsProjective,
//     ) -> (EdwardsProjective, EdwardsProjective) {
//         let mut rng = ark_std::test_rng();
//         let r = Fr::rand(&mut rng);
//         let base_affine = EdwardsAffine::generator();
//         let base_point: EdwardsProjective = base_affine.into();

//         let r_g = base_point * r;
//         let r_pk = pk * r;

//         let c0 = r_g + i_c0;
//         let c1 = r_pk + i_c1;

//         (c0, c1)
//     }

//     #[test]
//     fn test_decrypt() {
//         let mut secret_keys = Vec::new();
//         let mut public_keys = Vec::new();

//         for _ in 0..3 {
//             let (sk, pk) = key_gen();
//             secret_keys.push(sk);
//             public_keys.push(pk);
//         }
//         let agg_pk = public_keys.iter().copied().reduce(|a, b| a + b).unwrap();
//         let card_map = init_deck_and_card_map();

//         let (point, card) = card_map.iter().next().unwrap();

//         let mut c0 = EdwardsProjective::zero();
//         let mut c1 = *point;

//         for _ in 0..3 {
//             let (i_c0, i_c1) = elgamal_encrypt(agg_pk, c0, c1);
//             c0 = i_c0;
//             c1 = i_c1;
//         }

//         let mut partial_decryptions = Vec::new();
//         for i in 0..3 {
//             let sk_c0 = -(c0 * secret_keys[i]);
//             let x = sk_c0.x.to_bytes_le();
//             let y = sk_c0.y.to_bytes_le();
//             let z = sk_c0.z.to_bytes_le();
//             partial_decryptions.push([x, y, z]);
//         }

//         let c0x = c0.x.to_bytes_le();
//         let c0y = c0.y.to_bytes_le();
//         let c0z = c0.z.to_bytes_le();
//         let c1x = c1.x.to_bytes_le();
//         let c1y = c1.y.to_bytes_le();
//         let c1z = c1.z.to_bytes_le();

//         let encrypted_card = EncryptedCard {
//             c0: [c0x, c0y, c0z],
//             c1: [c1x, c1y, c1z],
//         };
//         let decrypted_point = decrypt_point(&card_map, &encrypted_card, partial_decryptions)
//             .expect("Point is not found");
//         assert_eq!(decrypted_point, *card);
//     }

//     #[test]
//     fn test_init_deck_and_card_map() {
//         let card_map = init_deck_and_card_map();
//         let json_cards: Vec<JsonCard> =
//             serde_json::from_str(CARD_MAP_JSON).expect("Failed to parse CARD_MAP_JSON");

//         assert_eq!(card_map.len(), 52, "Card map must contain 52 cards");
//         let mut json_map = HashMap::new();

//         for jc in json_cards {
//             let x_bytes = decimal_str_to_bytes_32_be(&jc.point.X);
//             let y_bytes = decimal_str_to_bytes_32_be(&jc.point.Y);
//             let z_bytes = decimal_str_to_bytes_32_be(&jc.point.Z);

//             let x = Fq::from_be_bytes_mod_order(&x_bytes);
//             let y = Fq::from_be_bytes_mod_order(&y_bytes);
//             let z = Fq::from_be_bytes_mod_order(&z_bytes);
//             let t = x * y;

//             let point = EdwardsProjective::new(x, y, t, z);

//             let rank = match jc.rank.as_str() {
//                 "2" => 2,
//                 "3" => 3,
//                 "4" => 4,
//                 "5" => 5,
//                 "6" => 6,
//                 "7" => 7,
//                 "8" => 8,
//                 "9" => 9,
//                 "10" => 10,
//                 "J" => 11,
//                 "Q" => 12,
//                 "K" => 13,
//                 "A" => 14,
//                 other => panic!("Unknown rank"),
//             };
//             let suit = match jc.suit.as_str() {
//                 "hearts" => Suit::Hearts,
//                 "diamonds" => Suit::Diamonds,
//                 "clubs" => Suit::Clubs,
//                 "spades" => Suit::Spades,
//                 _ => panic!("Unknown suit"),
//             };

//             json_map.insert(point, Card::new(suit, rank));
//         }
//         assert_eq!(card_map, json_map, "Rust-generated map must match JSON");
//     }
// }

pub fn get_decrypted_points(
    proofs: &[VerificationVariables],
    encrypted_cards: &HashMap<ActorId, [EncryptedCard; 2]>,
) -> Vec<(ActorId, [EncryptedCard; 2])> {
    let grouped = group_partial_decryptions_by_c0(proofs);

    let mut results = HashMap::new();

    for (c0_coords, partials) in grouped {
        let c1_coords = encrypted_cards
            .iter()
            .flat_map(|(_, cards)| cards)
            .find(|c| compare_points(&c.c0, &c0_coords))
            .map(|c| &c.c1);
        let Some(c1_coords) = c1_coords else {
            panic!("Missing c1")
        };
        let decryption_sum = sum_partial_decryptions(&partials);
        let decrypted = deserialize_bandersnatch_coords(c1_coords) + decryption_sum;

        results.insert(c0_coords, decrypted);
    }

    let mut cards_by_player = Vec::new();

    let empty_card = EncryptedCard {
        c0: [vec![], vec![], vec![]],
        c1: [vec![], vec![], vec![]],
    };

    for (actor_id, cards) in encrypted_cards {
        let mut decrypted_cards = [empty_card.clone(), empty_card.clone()];

        for (i, card) in cards.iter().enumerate() {
            let decrypted_point = results.get(&card.c0);
            let Some(decrypted_point) = decrypted_point else {
                panic!("Missing decryption");
            };

            decrypted_cards[i] = EncryptedCard {
                c0: card.c0.clone(),
                c1: serialize_bandersnatch_coords(decrypted_point),
            };
        }

        cards_by_player.push((*actor_id, decrypted_cards));
    }

    cards_by_player
}

fn group_partial_decryptions_by_c0(
    proofs: &[VerificationVariables],
) -> HashMap<[Vec<u8>; 3], Vec<[Vec<u8>; 3]>> {
    let mut map = HashMap::new();

    for proof in proofs {
        let (c0, expected) = parse_partial_decryption_inputs(&proof.public_input);
        map.entry(c0).or_insert_with(Vec::new).push(expected);
    }

    map
}

pub fn get_cards_and_decryptions(
    table_cards: &[EncryptedCard],
    proofs: &[VerificationVariables],
) -> Vec<(EncryptedCard, [Vec<u8>; 3])> {
    let mut decryptions = Vec::new();
    for proof in proofs {
        let (c0, dec) = parse_partial_decryption_inputs(&proof.public_input);
        let c1 = table_cards
            .iter()
            .find(|c| compare_points(&c.c0, &c0))
            .map(|c| &c.c1);
        let Some(c1) = c1 else {
            panic!("Card not found")
        };
        let card = EncryptedCard { c0, c1: c1.clone() };
        decryptions.push((card, dec));
    }
    decryptions
}
fn parse_partial_decryption_inputs(public_input: &[Vec<u8>]) -> ([Vec<u8>; 3], [Vec<u8>; 3]) {
    let is_valid = Fq::from_le_bytes_mod_order(&public_input[0]);

    if is_valid != Fq::one() {
        panic!("Invalid proof");
    }
    let c0 = [
        public_input[1].clone(),
        public_input[2].clone(),
        public_input[3].clone(),
    ];
    let expected = [
        public_input[4].clone(),
        public_input[5].clone(),
        public_input[6].clone(),
    ];
    (c0, expected)
}

fn sum_partial_decryptions(partials: &[[Vec<u8>; 3]]) -> EdwardsProjective {
    partials
        .iter()
        .fold(EdwardsProjective::zero(), |acc, coord| {
            let p = deserialize_bandersnatch_coords(coord);
            acc + p
        })
}
