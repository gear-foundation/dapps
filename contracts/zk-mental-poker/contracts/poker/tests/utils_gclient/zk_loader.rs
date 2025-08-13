#![allow(clippy::type_complexity)]
use ark_bls12_381::Bls12_381;
use ark_bls12_381::{Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_ec::pairing::Pairing;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use num_bigint::BigUint;
use num_traits::Num;
use poker_client::{EncryptedCard, ProofBytes, VerificationVariables, ZkPublicKey};
use serde::Deserialize;
use std::fs;
use std::ops::Neg;
use std::str::FromStr;
use zk_verification_client::VerifyingKeyBytes;

// === Constants ===
const FIELD_ELEMENT_SIZE: usize = 32;
const G1_UNCOMPRESSED_SIZE: usize = 96;
const EXPECTED_DECK_ROWS: usize = 6;
const DECK_SIZE: usize = 52;

// === Data Structures ===
#[derive(Debug, Deserialize)]
struct PlayerDecryptions {
    #[serde(rename = "playerPubKey")]
    player_pub_key: ECPointJson,
    decryptions: Vec<Decryption>,
}

#[derive(Debug, Deserialize)]
struct Decryption {
    #[serde(rename = "encryptedCard")]
    encrypted_card: EncryptedCardJson,
    dec: ECPointJson,
    proof: ProofJson,
    #[serde(rename = "publicSignals")]
    public_signals: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EncryptedCardJson {
    c0: ECPointJson,
    c1: ECPointJson,
}

#[derive(Debug, Deserialize)]
struct PublicKeyJson {
    index: usize,
    pk: ECPointJson,
}

#[derive(Debug, Deserialize)]
struct ECPointJson {
    #[serde(rename = "X")]
    x: String,
    #[serde(rename = "Y")]
    y: String,
    #[serde(rename = "Z")]
    z: String,
}

impl From<ECPointJson> for [Vec<u8>; 3] {
    fn from(p: ECPointJson) -> Self {
        [
            decimal_string_to_bytes(&p.x),
            decimal_string_to_bytes(&p.y),
            decimal_string_to_bytes(&p.z),
        ]
    }
}

#[derive(Debug, Deserialize)]
struct ProofJson {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
}

#[derive(Deserialize)]
struct BatchProofEntry {
    proof: ProofJson,
    #[serde(rename = "publicSignals")]
    public_signals: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct VKey {
    pub vk_alpha_1: Vec<String>,
    pub vk_beta_2: Vec<Vec<String>>,
    pub vk_gamma_2: Vec<Vec<String>>,
    pub vk_delta_2: Vec<Vec<String>>,
    #[serde(rename = "IC")]
    pub ic: Vec<Vec<String>>,
}

#[derive(Deserialize)]
struct JsonDecryptionEntry {
    #[serde(rename = "publicKey")]
    public_key: ECPointJson,
    cards: Vec<JsonCardProof>,
}

#[derive(Deserialize)]
struct JsonCardProof {
    decrypted: ECPointJson,
    proof: ProofJson,
    #[serde(rename = "publicSignals")]
    public_signals: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DecryptedCardWithProof {
    pub decrypted: [Vec<u8>; 3],
    pub proof: VerificationVariables,
}

#[derive(Debug)]
pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}
pub struct ZkLoaderData;

impl ZkLoaderData {
    pub fn load_verifying_key(path: &str) -> VerifyingKeyBytes {
        let vkey: VKey = Self::read_json(path);

        Self::construct_verifying_key(&vkey)
    }

    pub fn load_player_public_keys(path: &str) -> Vec<(usize, ZkPublicKey)> {
        Self::read_json::<Vec<PublicKeyJson>>(path)
            .into_iter()
            .map(|pk| (pk.index, ECPointConverter::to_public_key(&pk.pk)))
            .collect()
    }
    pub fn load_partial_decrypt_proofs(path: &str) -> Vec<VerificationVariables> {
        Self::load_batch_proofs(path)
    }

    pub fn load_shuffle_proofs(path: &str) -> Vec<VerificationVariables> {
        Self::load_batch_proofs(path)
    }

    pub fn load_table_cards_proofs(
        path: &str,
    ) -> Vec<(
        ZkPublicKey,
        (
            Vec<(EncryptedCard, [Vec<u8>; 3])>,
            Vec<VerificationVariables>,
        ),
    )> {
        let parsed: Vec<PlayerDecryptions> = Self::read_json(path);
        parsed
            .into_iter()
            .map(Self::process_player_decryptions)
            .collect()
    }

    pub fn load_cards_with_proofs(path: &str) -> Vec<(ZkPublicKey, Vec<DecryptedCardWithProof>)> {
        let parsed: Vec<JsonDecryptionEntry> = Self::read_json(path);
        parsed
            .into_iter()
            .map(Self::process_card_decryption_entry)
            .collect()
    }

    pub fn load_encrypted_table_cards(path: &str) -> Vec<EncryptedCard> {
        let json: Vec<Vec<String>> = Self::read_json(path);

        if json.len() != EXPECTED_DECK_ROWS {
            panic!(
                "Expected {} rows for encrypted deck, got {}",
                EXPECTED_DECK_ROWS,
                json.len()
            );
        }

        (0..DECK_SIZE)
            .map(|i| EncryptedCard {
                c0: [
                    decimal_string_to_bytes(&json[0][i]),
                    decimal_string_to_bytes(&json[1][i]),
                    decimal_string_to_bytes(&json[2][i]),
                ],
                c1: [
                    decimal_string_to_bytes(&json[3][i]),
                    decimal_string_to_bytes(&json[4][i]),
                    decimal_string_to_bytes(&json[5][i]),
                ],
            })
            .collect()
    }
    // === Private Methods ===
    fn construct_verifying_key(vkey: &VKey) -> VerifyingKeyBytes {
        let alpha = CurvePointDeserializer::deserialize_g1(&vkey.vk_alpha_1);
        let beta = CurvePointDeserializer::deserialize_g2(&vkey.vk_beta_2);
        let gamma = CurvePointDeserializer::deserialize_g2(&vkey.vk_gamma_2);
        let delta = CurvePointDeserializer::deserialize_g2(&vkey.vk_delta_2);
        let ic_points: Vec<G1Affine> = vkey
            .ic
            .iter()
            .map(|x| CurvePointDeserializer::deserialize_g1(x))
            .collect();

        let alpha_beta_pairing = Bls12_381::pairing(alpha, beta).0;
        let alpha_g1_beta_g2 = Self::serialize_uncompressed(&alpha_beta_pairing);
        let gamma_neg = Self::serialize_uncompressed(&gamma.into_group().neg().into_affine());
        let delta_neg = Self::serialize_uncompressed(&delta.into_group().neg().into_affine());

        let ic_uncompressed = ic_points
            .into_iter()
            .map(|p| {
                let bytes = Self::serialize_uncompressed(&p);
                assert_eq!(
                    bytes.len(),
                    G1_UNCOMPRESSED_SIZE,
                    "IC point must be {} bytes",
                    G1_UNCOMPRESSED_SIZE
                );
                bytes
            })
            .collect();

        VerifyingKeyBytes {
            alpha_g1_beta_g2,
            gamma_g2_neg_pc: gamma_neg,
            delta_g2_neg_pc: delta_neg,
            ic: ic_uncompressed,
        }
    }

    fn load_batch_proofs(path: &str) -> Vec<VerificationVariables> {
        let parsed: Vec<BatchProofEntry> = Self::read_json(path);

        parsed
            .into_iter()
            .map(|entry| Self::construct_proof(&entry.proof, &entry.public_signals))
            .collect()
    }

    fn process_player_decryptions(
        entry: PlayerDecryptions,
    ) -> (
        ZkPublicKey,
        (
            Vec<(EncryptedCard, [Vec<u8>; 3])>,
            Vec<VerificationVariables>,
        ),
    ) {
        let player_key = ECPointConverter::to_public_key(&entry.player_pub_key);
        let mut decryptions = Vec::new();
        let mut proofs = Vec::new();

        for dec in entry.decryptions {
            let card = EncryptedCard {
                c0: dec.encrypted_card.c0.into(),
                c1: dec.encrypted_card.c1.into(),
            };
            let decrypted = dec.dec.into();
            let proof = ProofProcessor::create_proof(&dec.proof, &dec.public_signals);

            decryptions.push((card, decrypted));
            proofs.push(proof);
        }

        (player_key, (decryptions, proofs))
    }

    fn process_card_decryption_entry(
        entry: JsonDecryptionEntry,
    ) -> (ZkPublicKey, Vec<DecryptedCardWithProof>) {
        let player_key = ECPointConverter::to_public_key(&entry.public_key);
        let cards = entry
            .cards
            .into_iter()
            .map(|card| {
                let decrypted = card.decrypted.into();
                let proof = ProofProcessor::create_proof(&card.proof, &card.public_signals);
                DecryptedCardWithProof { decrypted, proof }
            })
            .collect();
        (player_key, cards)
    }

    fn construct_proof(proof_json: &ProofJson, signals: &[String]) -> VerificationVariables {
        let proof = Proof {
            a: deserialize_g1(&proof_json.pi_a),
            b: deserialize_g2(&proof_json.pi_b),
            c: deserialize_g1(&proof_json.pi_c),
        };
        let public_inputs = parse_public_signals(signals);
        VerificationVariables {
            proof_bytes: encode_proof(&proof),
            public_input: encode_inputs(&public_inputs),
        }
    }

    fn read_json<T: serde::de::DeserializeOwned>(path: &str) -> T {
        let raw = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", path));
        serde_json::from_str(&raw).unwrap_or_else(|_| panic!("Invalid JSON in {}", path))
    }

    fn serialize_uncompressed<T: CanonicalSerialize>(value: &T) -> Vec<u8> {
        let mut buf = Vec::new();
        value
            .serialize_uncompressed(&mut buf)
            .expect("Serialization failed");
        buf
    }
}

// === Helper Modules ===
struct ECPointConverter;

impl ECPointConverter {
    fn to_public_key(point: &ECPointJson) -> ZkPublicKey {
        ZkPublicKey {
            x: Self::decimal_str_to_bytes_32(&point.x),
            y: Self::decimal_str_to_bytes_32(&point.y),
            z: Self::decimal_str_to_bytes_32(&point.z),
        }
    }

    fn decimal_str_to_bytes_32(s: &str) -> [u8; 32] {
        let n = BigUint::from_str_radix(s, 10).expect("Invalid decimal number");
        let b = n.to_bytes_be();

        if b.len() > FIELD_ELEMENT_SIZE {
            panic!("Number too large for {} bytes", FIELD_ELEMENT_SIZE);
        }

        let mut buf = [0u8; FIELD_ELEMENT_SIZE];
        buf[FIELD_ELEMENT_SIZE - b.len()..].copy_from_slice(&b);
        buf.reverse();
        buf
    }
}

struct CurvePointDeserializer;

impl CurvePointDeserializer {
    fn deserialize_g1(point: &[String]) -> G1Affine {
        let x_biguint = BigUint::from_str_radix(&point[0], 10).expect("Invalid x coordinate");
        let y_biguint = BigUint::from_str_radix(&point[1], 10).expect("Invalid y coordinate");

        let mut x_bytes = [0u8; 48];
        let mut y_bytes = [0u8; 48];

        let x_b = x_biguint.to_bytes_be();
        let y_b = y_biguint.to_bytes_be();

        x_bytes[48 - x_b.len()..].copy_from_slice(&x_b);
        y_bytes[48 - y_b.len()..].copy_from_slice(&y_b);

        let x = Fq::from_be_bytes_mod_order(&x_bytes);
        let y = Fq::from_be_bytes_mod_order(&y_bytes);

        G1Affine::new(x, y)
    }

    pub fn deserialize_g2(coords: &[Vec<String>]) -> G2Affine {
        if coords.len() != 3 {
            panic!("G2Affine coordinates must have exactly 3 pairs");
        }

        if coords[2][0] != "1" || coords[2][1] != "0" {
            panic!("Expected third coordinate to be [1, 0] for affine representation");
        }

        let x_c0 = Fq::from_str(&coords[0][0]).expect("Invalid x_c1 coordinate");
        let x_c1 = Fq::from_str(&coords[0][1]).expect("Invalid x_c0 coordinate");

        let y_c0 = Fq::from_str(&coords[1][0]).expect("Invalid y_c1 coordinate");
        let y_c1 = Fq::from_str(&coords[1][1]).expect("Invalid y_c0 coordinate");

        let x = Fq2::new(x_c0, x_c1);
        let y = Fq2::new(y_c0, y_c1);

        G2Affine::new(x, y)
    }
}
pub fn deserialize_g1(point: &[String]) -> G1Affine {
    let x_biguint = BigUint::from_str_radix(&point[0], 10).unwrap();
    let y_biguint = BigUint::from_str_radix(&point[1], 10).unwrap();

    let mut x_bytes = [0u8; 48];
    let mut y_bytes = [0u8; 48];

    let x_b = x_biguint.to_bytes_be();
    let y_b = y_biguint.to_bytes_be();

    x_bytes[48 - x_b.len()..].copy_from_slice(&x_b);
    y_bytes[48 - y_b.len()..].copy_from_slice(&y_b);

    let x = Fq::from_be_bytes_mod_order(&x_bytes);
    let y = Fq::from_be_bytes_mod_order(&y_bytes);

    G1Affine::new(x, y)
}

struct ProofProcessor;

impl ProofProcessor {
    fn create_proof(proof_json: &ProofJson, signals: &[String]) -> VerificationVariables {
        let proof = Proof {
            a: CurvePointDeserializer::deserialize_g1(&proof_json.pi_a),
            b: CurvePointDeserializer::deserialize_g2(&proof_json.pi_b),
            c: CurvePointDeserializer::deserialize_g1(&proof_json.pi_c),
        };

        let public_inputs = Self::parse_public_signals(signals);

        VerificationVariables {
            proof_bytes: Self::encode_proof(&proof),
            public_input: Self::encode_inputs(&public_inputs),
        }
    }

    fn parse_public_signals(signals: &[String]) -> Vec<Fr> {
        signals
            .iter()
            .map(|s| {
                let n = BigUint::from_str_radix(s, 10).expect("Invalid public signal");
                let bytes = n.to_bytes_le();
                let mut buf = [0u8; FIELD_ELEMENT_SIZE];
                buf[..bytes.len()].copy_from_slice(&bytes);
                Fr::from_le_bytes_mod_order(&buf)
            })
            .collect()
    }

    fn encode_proof(proof: &Proof) -> ProofBytes {
        let mut a_bytes = Vec::new();
        let mut b_bytes = Vec::new();
        let mut c_bytes = Vec::new();

        proof
            .a
            .serialize_uncompressed(&mut a_bytes)
            .expect("Failed to serialize proof component A");
        proof
            .b
            .serialize_uncompressed(&mut b_bytes)
            .expect("Failed to serialize proof component B");
        proof
            .c
            .serialize_uncompressed(&mut c_bytes)
            .expect("Failed to serialize proof component C");

        ProofBytes {
            a: a_bytes,
            b: b_bytes,
            c: c_bytes,
        }
    }

    fn encode_inputs(inputs: &[Fr]) -> Vec<Vec<u8>> {
        inputs
            .iter()
            .map(|fr| {
                let mut buf = Vec::new();
                fr.serialize_uncompressed(&mut buf)
                    .expect("Failed to serialize field element");
                buf
            })
            .collect()
    }
}
pub fn deserialize_g2(coords: &[Vec<String>]) -> G2Affine {
    if coords.len() != 3 {
        panic!("G2Affine coordinates must have exactly 3 pairs");
    }

    if coords[2][0] != "1" || coords[2][1] != "0" {
        panic!("Expected third coordinate to be [1, 0] for affine representation");
    }

    let x_c0 = Fq::from_str(&coords[0][0]).expect("Invalid x_c1 coordinate");
    let x_c1 = Fq::from_str(&coords[0][1]).expect("Invalid x_c0 coordinate");

    let y_c0 = Fq::from_str(&coords[1][0]).expect("Invalid y_c1 coordinate");
    let y_c1 = Fq::from_str(&coords[1][1]).expect("Invalid y_c0 coordinate");

    let x = Fq2::new(x_c0, x_c1);
    let y = Fq2::new(y_c0, y_c1);

    G2Affine::new(x, y)
}

fn parse_public_signals(signals: &[String]) -> Vec<Fr> {
    signals
        .iter()
        .map(|s| {
            let n = BigUint::from_str_radix(s, 10).unwrap();
            let bytes = n.to_bytes_le();
            let mut buf = [0u8; 32];
            buf[..bytes.len()].copy_from_slice(&bytes);
            Fr::from_le_bytes_mod_order(&buf)
        })
        .collect()
}

fn encode_proof(proof: &Proof) -> ProofBytes {
    let mut a_bytes = Vec::new();
    let mut b_bytes = Vec::new();
    let mut c_bytes = Vec::new();

    proof.a.serialize_uncompressed(&mut a_bytes).unwrap();
    proof.b.serialize_uncompressed(&mut b_bytes).unwrap();
    proof.c.serialize_uncompressed(&mut c_bytes).unwrap();

    ProofBytes {
        a: a_bytes,
        b: b_bytes,
        c: c_bytes,
    }
}

fn encode_inputs(inputs: &[Fr]) -> Vec<Vec<u8>> {
    inputs
        .iter()
        .map(|fr| {
            let mut buf = Vec::new();
            fr.serialize_uncompressed(&mut buf).unwrap();
            buf
        })
        .collect()
}

// === Utility Functions ===
fn decimal_string_to_bytes(s: &str) -> Vec<u8> {
    let n = BigUint::from_str_radix(s, 10).expect("Invalid decimal number");
    let mut b = n.to_bytes_le();
    b.resize(FIELD_ELEMENT_SIZE, 0);
    b
}
