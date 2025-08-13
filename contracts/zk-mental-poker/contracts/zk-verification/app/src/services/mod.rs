#![allow(static_mut_refs)]
use core::ops::AddAssign;
use gbuiltin_bls381::{
    Request, Response,
    ark_bls12_381::{Bls12_381, Fr, G1Affine, G1Projective as G1, G2Affine},
    ark_ec::{AffineRepr, Group, pairing::Pairing},
    ark_ff::Field,
    ark_scale,
    ark_scale::hazmat::ArkScaleProjective,
    ark_serialize::CanonicalDeserialize,
};
use sails_rs::{ActorId, Encode, gstd::msg, prelude::*};

const ACTOR_ID: [u8; 32] =
    hex_literal::hex!("6b6e292c382945e80bf51af2ba7fe9f458dcff81ae6075c46f9095e1bbecdc37");

static mut STORAGE: Option<Storage> = None;

#[derive(Debug)]
struct Storage {
    shuffle_verification_context: BatchVerificationContext,
    decrypt_verificaiton_context: BatchVerificationContext,
}
pub struct ZkVerificationService(());

impl ZkVerificationService {
    fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}
#[sails_rs::service]
#[allow(clippy::new_without_default)]
impl ZkVerificationService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn init(vk_shuffle_bytes: VerifyingKeyBytes, vk_decrypt_bytes: VerifyingKeyBytes) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                decrypt_verificaiton_context: BatchVerificationContext::new(
                    &vk_decrypt_bytes,
                    ActorId::from(ACTOR_ID),
                ),
                shuffle_verification_context: BatchVerificationContext::new(
                    &vk_shuffle_bytes,
                    ActorId::from(ACTOR_ID),
                ),
            });
        }
        Self(())
    }

    pub async fn verify_shuffle(&mut self, instances: Vec<VerificationVariables>) {
        let storage = self.get();
        storage
            .shuffle_verification_context
            .verify_batch(instances)
            .await;
    }

    pub async fn verify_decrypt(&mut self, instances: Vec<VerificationVariables>) {
        let storage = self.get();
        storage
            .decrypt_verificaiton_context
            .verify_batch(instances)
            .await;
    }
}

// ================================================================================================
// Type Aliases & Constants
// ================================================================================================

type ArkScale<T> = ark_scale::ArkScale<T, { ark_scale::HOST_CALL }>;
type Gt = <Bls12_381 as Pairing>::TargetField;

// ================================================================================================
// Core Data Structures
// ================================================================================================

/// Serialized verifying key for zk-SNARK verification
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = scale_info)]
pub struct VerifyingKeyBytes {
    pub alpha_g1_beta_g2: Vec<u8>,
    pub gamma_g2_neg_pc: Vec<u8>,
    pub delta_g2_neg_pc: Vec<u8>,
    pub ic: Vec<Vec<u8>>,
}

/// Deserialized verifying key with curve points
#[derive(Debug, Clone)]
pub struct VerifyingKey {
    pub alpha_g1_beta_g2: ArkScale<Gt>,
    pub gamma_g2_neg_pc: G2Affine,
    pub delta_g2_neg_pc: G2Affine,
    pub ic: Vec<G1Affine>,
}

/// Serialized zk-SNARK proof components
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = scale_info)]
pub struct ProofBytes {
    pub a: Vec<u8>,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}

/// Helper struct for managing proof point collections
#[derive(Debug, Clone)]
struct ProofPoints {
    a_points: Vec<G1Affine>,
    b_points: Vec<G2Affine>,
    batch_size: usize,
}

/// Deserialized proof components
#[derive(Debug, Clone)]
struct ProofComponents {
    a: G1Affine,
    b: G2Affine,
    c: G1Affine,
}

impl ProofComponents {
    fn from_bytes(proof_bytes: &ProofBytes) -> Self {
        Self {
            a: CurvePointDeserializer::deserialize_g1(&proof_bytes.a),
            b: CurvePointDeserializer::deserialize_g2(&proof_bytes.b),
            c: CurvePointDeserializer::deserialize_g1(&proof_bytes.c),
        }
    }
}

/// Complete verification instance containing proof and public inputs
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = scale_info)]
pub struct VerificationVariables {
    pub proof_bytes: ProofBytes,
    pub public_input: Vec<Vec<u8>>,
}

/// Batch verification context for managing multiple proof verifications
#[derive(Debug)]
pub struct BatchVerificationContext {
    pub verifying_key: VerifyingKey,
    pub builtin_address: ActorId,
}

// ================================================================================================
// Core Implementation
// ================================================================================================

impl VerifyingKey {
    /// Creates a new verifying key from serialized bytes
    pub fn from_bytes(vk_bytes: &VerifyingKeyBytes) -> Self {
        Self {
            delta_g2_neg_pc: CurvePointDeserializer::deserialize_g2(&vk_bytes.delta_g2_neg_pc),
            gamma_g2_neg_pc: CurvePointDeserializer::deserialize_g2(&vk_bytes.gamma_g2_neg_pc),
            alpha_g1_beta_g2: ArkScale::decode(&mut &*vk_bytes.alpha_g1_beta_g2)
                .expect("Deserialization failed: alpha_g1_beta_g2"),
            ic: vk_bytes
                .ic
                .iter()
                .map(|bytes| CurvePointDeserializer::deserialize_g1(bytes))
                .collect(),
        }
    }
}

impl BatchVerificationContext {
    /// Creates a new batch verification context
    pub fn new(vk_bytes: &VerifyingKeyBytes, builtin_address: ActorId) -> Self {
        BatchVerificationContext {
            verifying_key: VerifyingKey::from_bytes(vk_bytes),
            builtin_address,
        }
    }

    /// Performs batch verification of multiple zk-SNARK proofs
    pub async fn verify_batch(&self, instances: Vec<VerificationVariables>) {
        let proof_points = self.prepare_proof_points(instances).await;
        let is_valid = self.execute_pairing_check(&proof_points).await;

        if !is_valid {
            panic!("Batch verification failed");
        }
    }

    /// Prepares proof points for batch verification
    async fn prepare_proof_points(&self, instances: Vec<VerificationVariables>) -> ProofPoints {
        let len = instances.len();
        let mut a_points = Vec::with_capacity(3 * len);
        let mut b_points = Vec::with_capacity(3 * len);

        for instance in instances.into_iter() {
            let prepared_inputs = PublicInputProcessor::prepare_inputs_bytes(
                &instance.public_input,
                &self.verifying_key.ic,
                self.builtin_address,
            )
            .await;

            let proof_components = ProofComponents::from_bytes(&instance.proof_bytes);

            a_points.extend([proof_components.a, prepared_inputs, proof_components.c]);
            b_points.extend([
                proof_components.b,
                self.verifying_key.gamma_g2_neg_pc,
                self.verifying_key.delta_g2_neg_pc,
            ]);
        }

        ProofPoints {
            a_points,
            b_points,
            batch_size: len,
        }
    }

    /// Executes the pairing check for batch verification
    async fn execute_pairing_check(&self, proof_points: &ProofPoints) -> bool {
        let a: ArkScale<Vec<G1Affine>> = proof_points.a_points.clone().into();
        let b: ArkScale<Vec<G2Affine>> = proof_points.b_points.clone().into();

        let miller_out =
            PairingOperations::multi_miller_loop(a.encode(), b.encode(), self.builtin_address)
                .await;

        let exp = PairingOperations::final_exponentiation(miller_out, self.builtin_address).await;
        let expected = self
            .verifying_key
            .alpha_g1_beta_g2
            .0
            .pow([proof_points.batch_size as u64]);

        exp.0 == expected
    }
}

/// Handles pairing operations via builtin calls
struct PairingOperations;

impl PairingOperations {
    async fn multi_scalar_mul_g1(
        bases: Vec<u8>,
        scalars: Vec<u8>,
        builtin_address: ActorId,
    ) -> Vec<u8> {
        match PairingOperations::send_request_and_extract(
            Request::MultiScalarMultiplicationG1 { bases, scalars },
            builtin_address,
            "MSM",
        )
        .await
        {
            Response::MultiScalarMultiplicationG1(result) => result,
            _ => unreachable!("MSM: unexpected response type"),
        }
    }

    async fn multi_miller_loop(g1: Vec<u8>, g2: Vec<u8>, builtin_address: ActorId) -> Vec<u8> {
        match PairingOperations::send_request_and_extract(
            Request::MultiMillerLoop { a: g1, b: g2 },
            builtin_address,
            "MultiMillerLoop",
        )
        .await
        {
            Response::MultiMillerLoop(result) => result,
            _ => unreachable!("MultiMillerLoop: unexpected response type"),
        }
    }

    async fn final_exponentiation(f: Vec<u8>, builtin_address: ActorId) -> ArkScale<Gt> {
        match PairingOperations::send_request_and_extract(
            Request::FinalExponentiation { f },
            builtin_address,
            "FinalExp",
        )
        .await
        {
            Response::FinalExponentiation(result) => ArkScale::<Gt>::decode(&mut result.as_slice())
                .expect("FinalExp: decode ArkScale failed"),
            _ => unreachable!("FinalExp: unexpected response type"),
        }
    }

    async fn send_request_and_extract(
        request: Request,
        builtin_address: ActorId,
        context: &'static str,
    ) -> Response {
        let reply = msg::send_bytes_for_reply(builtin_address, request.encode(), 0, 0)
            .unwrap_or_else(|_| panic!("{}: failed to send request", context))
            .await
            .unwrap_or_else(|_| panic!("{}: reply failed", context));

        Response::decode(&mut reply.as_slice())
            .unwrap_or_else(|_| panic!("{}: failed to decode response", context))
    }
}

/// Handles public input processing and preparation
struct PublicInputProcessor;

impl PublicInputProcessor {
    /// Prepares public inputs for verification from byte representation
    async fn prepare_inputs_bytes(
        public_input: &[Vec<u8>],
        ic: &[G1Affine],
        builtin_address: ActorId,
    ) -> G1Affine {
        let public_inputs: Vec<Fr> = public_input
            .iter()
            .map(|bytes| {
                Fr::deserialize_uncompressed_unchecked(&**bytes)
                    .expect("Deserialization failed: public input")
            })
            .collect();

        Self::prepare_inputs(ic, &public_inputs, builtin_address).await
    }

    /// Prepares verification inputs using multi-scalar multiplication
    async fn prepare_inputs(
        gamma_abc_g1: &[G1Affine],
        public_inputs: &[Fr],
        builtin_address: ActorId,
    ) -> G1Affine {
        if (public_inputs.len() + 1) != gamma_abc_g1.len() {
            panic!("Invalid proof length");
        }

        let mut g_ic = gamma_abc_g1[0].into_group();

        let bases: ArkScale<Vec<G1Affine>> = gamma_abc_g1[1..].to_vec().into();
        let scalars: ArkScale<Vec<<G1 as Group>::ScalarField>> = public_inputs.to_vec().into();

        let msm_result_bytes = PairingOperations::multi_scalar_mul_g1(
            bases.encode(),
            scalars.encode(),
            builtin_address,
        )
        .await;

        let msm_result_affine = ArkScaleProjective::<G1>::decode(&mut msm_result_bytes.as_slice())
            .expect("Deserialization failed: MSM result")
            .0;

        g_ic.add_assign(msm_result_affine);
        g_ic.into()
    }
}

// ================================================================================================
// Utility Modules
// ================================================================================================

/// Handles curve point deserialization
struct CurvePointDeserializer;

impl CurvePointDeserializer {
    #[inline]
    fn deserialize_g1(data: &[u8]) -> G1Affine {
        G1Affine::deserialize_uncompressed_unchecked(data)
            .expect("Deserialization failed: G1 point")
    }

    #[inline]
    fn deserialize_g2(data: &[u8]) -> G2Affine {
        G2Affine::deserialize_uncompressed_unchecked(data)
            .expect("Deserialization failed: G2 point")
    }
}
