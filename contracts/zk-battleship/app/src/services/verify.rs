use core::ops::AddAssign;
use gbuiltin_bls381::{
    Request, Response,
    ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine},
    ark_ec::{AffineRepr, pairing::Pairing},
    ark_ff::PrimeField,
    ark_scale,
    ark_serialize::{CanonicalDeserialize, CanonicalSerialize},
};
use gstd::{ActorId, Encode, ext, msg, prelude::*};

type ArkScale<T> = ark_scale::ArkScale<T, { ark_scale::HOST_CALL }>;

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct VerifyingKeyBytes {
    pub alpha_g1_beta_g2: Vec<u8>,
    pub gamma_g2_neg_pc: Vec<u8>,
    pub delta_g2_neg_pc: Vec<u8>,
    pub ic: Vec<Vec<u8>>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ProofBytes {
    pub a: Vec<u8>,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PublicMoveInput {
    pub out: u8,
    pub hit: u8,
    pub hash: Vec<u8>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct VerificationVariables {
    pub proof_bytes: ProofBytes,
    pub public_input: PublicMoveInput,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct VerificationResult {
    pub res: u8,
    pub hit: u8,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PublicStartInput {
    pub hash: Vec<u8>,
}

pub async fn verify(
    vk: &VerifyingKeyBytes,
    proof: ProofBytes,
    prepared_inputs_bytes: Vec<u8>,
    builtin_bls381_address: ActorId,
) {
    let alpha_g1_beta_g2 = <ArkScale<<Bls12_381 as Pairing>::TargetField> as Decode>::decode(
        &mut vk.alpha_g1_beta_g2.as_slice(),
    )
    .expect("Decode error");
    let gamma_g2_neg_pc = G2Affine::deserialize_uncompressed_unchecked(&*vk.gamma_g2_neg_pc)
        .expect("Deserialize error");
    let delta_g2_neg_pc = G2Affine::deserialize_uncompressed_unchecked(&*vk.delta_g2_neg_pc)
        .expect("Deserialize error");
    let a = G1Affine::deserialize_uncompressed_unchecked(&*proof.a).expect("Deserialize error");

    let b = G2Affine::deserialize_uncompressed_unchecked(&*proof.b).expect("Deserialize error");

    let c = G1Affine::deserialize_uncompressed_unchecked(&*proof.c).expect("Deserialize error");
    let prepared_inputs = G1Affine::deserialize_uncompressed_unchecked(&*prepared_inputs_bytes)
        .expect("Deserialize error");

    let a: ArkScale<Vec<G1Affine>> = vec![a, prepared_inputs, c].into();
    let b: ArkScale<Vec<G2Affine>> = vec![b, gamma_g2_neg_pc, delta_g2_neg_pc].into();

    let miller_out =
        calculate_multi_miller_loop(a.encode(), b.encode(), builtin_bls381_address).await;

    let exp = calculate_exponentiation(miller_out, builtin_bls381_address).await;

    if exp != alpha_g1_beta_g2 {
        ext::panic("Verification failed");
    }
}

async fn calculate_multi_miller_loop(
    g1: Vec<u8>,
    g2: Vec<u8>,
    builtin_bls381_address: ActorId,
) -> Vec<u8> {
    let request = Request::MultiMillerLoop { a: g1, b: g2 }.encode();
    let reply = msg::send_bytes_for_reply(builtin_bls381_address, &request, 0, 0)
        .expect("Failed to send message")
        .await
        .expect("Received error reply");
    let response = Response::decode(&mut reply.as_slice()).expect("Error: decode response");
    match response {
        Response::MultiMillerLoop(v) => v,
        _ => unreachable!(),
    }
}

async fn calculate_exponentiation(
    f: Vec<u8>,
    builtin_bls381_address: ActorId,
) -> ArkScale<<Bls12_381 as Pairing>::TargetField> {
    let request = Request::FinalExponentiation { f }.encode();
    let reply = msg::send_bytes_for_reply(builtin_bls381_address, &request, 0, 0)
        .expect("Failed to send message")
        .await
        .expect("Received error reply");
    let response = Response::decode(&mut reply.as_slice()).expect("Error: decode response");
    match response {
        Response::FinalExponentiation(v) => {
            ArkScale::<<Bls12_381 as Pairing>::TargetField>::decode(&mut v.as_slice())
                .expect("Error: decode ArkScale")
        }
        _ => unreachable!(),
    }
}

pub fn get_move_prepared_inputs_bytes(public_input: PublicMoveInput, ic: Vec<Vec<u8>>) -> Vec<u8> {
    let public_inputs: Vec<Fr> = vec![
        Fr::from(public_input.out),
        Fr::from(public_input.hit),
        Fr::deserialize_uncompressed_unchecked(&*public_input.hash).expect("Deserialize error"),
    ];

    let gamma_abc_g1: Vec<G1Affine> = ic
        .into_iter()
        .map(|ic_element| {
            G1Affine::deserialize_uncompressed_unchecked(&*ic_element).expect("Deserialize error")
        })
        .collect();

    prepare_inputs(&gamma_abc_g1, &public_inputs)
}

pub fn get_start_prepared_inputs_bytes(
    public_input: PublicStartInput,
    ic: Vec<Vec<u8>>,
) -> Vec<u8> {
    let public_inputs: Vec<Fr> = vec![
        Fr::deserialize_uncompressed_unchecked(&*public_input.hash).expect("Deserialize error"),
    ];

    let gamma_abc_g1: Vec<G1Affine> = ic
        .into_iter()
        .map(|ic_element| {
            G1Affine::deserialize_uncompressed_unchecked(&*ic_element).expect("Deserialize error")
        })
        .collect();

    prepare_inputs(&gamma_abc_g1, &public_inputs)
}

fn prepare_inputs(gamma_abc_g1: &[G1Affine], public_inputs: &[Fr]) -> Vec<u8> {
    if (public_inputs.len() + 1) != gamma_abc_g1.len() {
        panic!("Wrong public inputs or IC length");
    }

    let mut g_ic = gamma_abc_g1[0].into_group();
    for (i, b) in public_inputs.iter().zip(gamma_abc_g1.iter().skip(1)) {
        g_ic.add_assign(&b.mul_bigint(i.into_bigint()));
    }

    let mut prepared_inputs_bytes = Vec::new();
    g_ic.serialize_uncompressed(&mut prepared_inputs_bytes)
        .expect("Deserialize error");

    prepared_inputs_bytes
}
