use crate::services::game::EdwardsProjective;
use crate::services::game::{
    EncryptedCard, ZkPublicKey,
    curve::{compare_points, compare_projective_and_coords, compare_public_keys},
};
use ark_ed_on_bls12_381_bandersnatch::Fq;
use ark_ff::{One, PrimeField};
use sails_rs::prelude::*;
use zk_verification_client::VerificationVariables;

/// Card deck configuration constants
#[derive(Debug, Clone, Copy)]
pub struct DeckConfig {
    pub num_cards: usize,
    pub num_coords: usize, // Coordinates per encrypted card point (X, Y, Z for both c0 and c1)
    pub pk_size: usize,    // Public key components count
}

impl DeckConfig {
    pub const STANDARD: Self = Self {
        num_cards: 52,
        num_coords: 6, // c0.X, c0.Y, c0.Z, c1.X, c1.Y, c1.Z
        pk_size: 3,    // x, y, z coordinates
    };

    #[inline]
    pub const fn expected_input_length(&self) -> usize {
        1 + self.pk_size + (self.num_coords * self.num_cards * 2) // valid + pk + original + permuted
    }
}

// ================================================================================================
// Core Data Structures
// ================================================================================================

/// Parsed public input containing original deck, permuted deck, and public key
#[derive(Debug, Clone)]
pub struct ParsedPublicInput {
    pub original_deck: Vec<EncryptedCard>,
    pub permuted_deck: Vec<EncryptedCard>,
    pub public_key: ZkPublicKey,
}

// ================================================================================================
// Utility Modules
// ================================================================================================

/// Handles parsing and validation of public inputs
struct PublicInputParser;

impl PublicInputParser {
    /// Parses original and permuted card decks from public input
    pub fn parse_original_and_permuted(
        public_input: &[Vec<u8>],
        config: DeckConfig,
    ) -> ParsedPublicInput {
        if public_input.len() != config.expected_input_length() {
            panic!("Invalid proof length");
        }

        // Validate proof validity flag
        let is_valid = Fq::from_le_bytes_mod_order(&public_input[0]);
        if is_valid != Fq::one() {
            panic!("Invalid proof flag");
        }

        let public_key = Self::extract_public_key(public_input);

        // Parse decks
        let original_offset = 1 + config.pk_size;
        let permuted_offset = original_offset + config.num_coords * config.num_cards;

        let original_deck = Self::parse_encrypted_deck(public_input, original_offset, config);
        let permuted_deck = Self::parse_encrypted_deck(public_input, permuted_offset, config);

        ParsedPublicInput {
            original_deck,
            permuted_deck,
            public_key,
        }
    }

    fn extract_public_key(public_input: &[Vec<u8>]) -> ZkPublicKey {
        ZkPublicKey {
            x: public_input[1]
                .clone()
                .try_into()
                .expect("Deserialization failed: pk.x"),
            y: public_input[2]
                .clone()
                .try_into()
                .expect("Deserialization failed: pk.y"),
            z: public_input[3]
                .clone()
                .try_into()
                .expect("Deserialization failed: pk.z"),
        }
    }

    fn parse_encrypted_deck(
        public_input: &[Vec<u8>],
        offset: usize,
        config: DeckConfig,
    ) -> Vec<EncryptedCard> {
        (0..config.num_cards)
            .map(|card_idx| EncryptedCard {
                c0: [
                    public_input[offset + card_idx].clone(),
                    public_input[offset + config.num_cards + card_idx].clone(),
                    public_input[offset + 2 * config.num_cards + card_idx].clone(),
                ],
                c1: [
                    public_input[offset + 3 * config.num_cards + card_idx].clone(),
                    public_input[offset + 4 * config.num_cards + card_idx].clone(),
                    public_input[offset + 5 * config.num_cards + card_idx].clone(),
                ],
            })
            .collect()
    }
}

/// Handles shuffle chain validation
pub struct ShuffleChainValidator;

impl ShuffleChainValidator {
    /// Validates the integrity of a shuffle chain
    pub fn validate_shuffle_chain(
        instances: &[VerificationVariables],
        original_deck: &[EdwardsProjective],
        expected_pub_key: &ZkPublicKey,
        final_encrypted_deck: &[EncryptedCard],
    ) {
        let config = DeckConfig::STANDARD;

        // Parse and validate first instance
        let first_parsed =
            PublicInputParser::parse_original_and_permuted(&instances[0].public_input, config);

        if !compare_public_keys(expected_pub_key, &first_parsed.public_key) {
            panic!("Public key mismatch");
        }

        Self::validate_initial_deck_matches(original_deck, &first_parsed.original_deck);

        let mut current_deck = first_parsed.permuted_deck;

        // Validate chain continuity
        for instance in instances[1..].iter() {
            let parsed =
                PublicInputParser::parse_original_and_permuted(&instance.public_input, config);

            if !compare_public_keys(expected_pub_key, &parsed.public_key) {
                panic!("Public key mismatch");
            }

            if parsed.original_deck != current_deck {
                panic!("Shuffle chain discontinuity");
            }

            current_deck = parsed.permuted_deck;
        }

        // Validate final deck state
        Self::validate_final_deck_matches(&current_deck, final_encrypted_deck);
    }

    fn validate_initial_deck_matches(expected: &[EdwardsProjective], actual: &[EncryptedCard]) {
        if expected.len() != actual.len() {
            panic!("Initial deck len mismatch");
        }

        for (expected_card, actual_card) in expected.iter().zip(actual) {
            if !compare_projective_and_coords(expected_card, &actual_card.c1) {
                panic!("Initial deck mismatch");
            }
        }
    }

    fn validate_final_deck_matches(expected: &[EncryptedCard], actual: &[EncryptedCard]) {
        for (expected_card, actual_card) in expected.iter().zip(actual) {
            if !compare_points(&expected_card.c0, &actual_card.c0)
                || !compare_points(&expected_card.c1, &actual_card.c1)
            {
                panic!("Final deck mismatch");
            }
        }
    }
}
