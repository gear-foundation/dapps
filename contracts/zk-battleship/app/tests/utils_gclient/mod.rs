use gclient::{EventProcessor, GearApi, Result};
use gear_core::ids::{MessageId, ProgramId};
use gstd::{ActorId, Decode, Encode};

use ark_std::ops::Neg;
use gbuiltin_bls381::ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use gbuiltin_bls381::ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};
use gbuiltin_bls381::ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use battleship::services::admin::storage::configuration::Configuration;
use battleship::services::multiple::{MultipleGame, MultipleGameState};
use battleship::services::single::SingleGameState;
use battleship::services::verify::VerifyingKeyBytes as InputVerifyingKeyBytes;
use battleship::services::verify::{ProofBytes, PublicMoveInput, PublicStartInput};
use hex_literal::hex;

pub const USERS_STR: &[&str] = &["//John", "//Mike", "//Dan"];

const BUILTIN_BLS381: ActorId = ActorId::new(hex!(
    "6b6e292c382945e80bf51af2ba7fe9f458dcff81ae6075c46f9095e1bbecdc37"
));

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }

    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
}

fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
}

#[derive(Debug, Clone)]
pub struct VerifyingKeyBytes {
    pub vk_alpha_g1: Vec<u8>,
    pub vk_beta_g2: Vec<u8>,
    pub vk_gamma_g2: Vec<u8>,
    pub vk_delta_g2: Vec<u8>,
}

#[macro_export]
macro_rules! send_request {
    (api: $api:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, payload: ($($val:expr),*)) => {
        $crate::send_request!(api: $api, program_id: $program_id, service_name: $name, action: $action, payload: ($($val),*), value: 0)
    };

    (api: $api:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, payload: ($($val:expr),*), value: $value:expr) => {
        {
            let request = [
                $name.encode(),
                $action.to_string().encode(),
                ($($val),*).encode(),
            ].concat();

            let gas_info = $api
                .calculate_handle_gas(None, $program_id, request.clone(), $value, true)
                .await?;

            let (message_id, _) = $api
                .send_message_bytes($program_id, request.clone(), gas_info.min_limit, $value)
                .await?;

            message_id
        }
    };
}

pub async fn init(
    api: &GearApi,
    start_vk: InputVerifyingKeyBytes,
    move_vk: InputVerifyingKeyBytes,
) -> (MessageId, ProgramId) {
    let config = Configuration {
        gas_for_check_time: 5_000_000_000,
        gas_for_delete_multiple_game: 5_000_000_000,
        gas_for_delete_single_game: 10_000_000_000,
        gas_for_delete_session: 5_000_000_000,
        delay_for_check_time: 20,             // 1 min
        delay_for_delete_multiple_game: 2400, // 2 hour
        delay_for_delete_single_game: 2400,   // 2 hour
        minimum_session_duration_ms: 180_000, // 3 mins
        block_duration_ms: 3_000,
    };
    let request = [
        "New".encode(),
        (BUILTIN_BLS381, start_vk, move_vk, config).encode(),
    ]
    .concat();
    let path = "../target/wasm32-unknown-unknown/release/battleship_wasm.opt.wasm";
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path).unwrap(),
            request.clone(),
            0,
            true,
        )
        .await
        .expect("Error calculate upload gas");

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path).unwrap(),
            gclient::now_micros().to_le_bytes(),
            request,
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error upload program bytes");

    (message_id, program_id)
}

pub async fn get_state_single_games(
    api: &GearApi,
    program_id: ProgramId,
    listener: &mut gclient::EventListener,
) -> Vec<(ActorId, SingleGameState)> {
    let request = ["Single".encode(), "Games".to_string().encode()].concat();

    let gas_info = api
        .calculate_handle_gas(None, program_id, request.clone(), 0, true)
        .await
        .expect("Error calculate handle gas");

    let (message_id, _) = api
        .send_message_bytes(program_id, request.clone(), gas_info.min_limit * 2, 0)
        .await
        .expect("Error send message bytes");

    let (_, raw_reply, _) = listener
        .reply_bytes_on(message_id)
        .await
        .expect("Error listen reply");

    let decoded_reply: (String, String, Vec<(ActorId, SingleGameState)>) = match raw_reply {
        Ok(raw_reply) => decode(raw_reply).expect("Erroe decode reply"),
        Err(_error) => gstd::panic!("Error in getting reply"),
    };
    // println!("decoded_reply {:?}", decoded_reply);
    decoded_reply.2
}

pub async fn get_state_multiple_games(
    api: &GearApi,
    program_id: ProgramId,
    listener: &mut gclient::EventListener,
) -> Vec<(ActorId, MultipleGameState)> {
    let request = ["Multiple".encode(), "Games".to_string().encode()].concat();

    let gas_info = api
        .calculate_handle_gas(None, program_id, request.clone(), 0, true)
        .await
        .expect("Error calculate handle gas");

    let (message_id, _) = api
        .send_message_bytes(program_id, request.clone(), gas_info.min_limit * 2, 0)
        .await
        .expect("Error send message bytes");

    let (_, raw_reply, _) = listener
        .reply_bytes_on(message_id)
        .await
        .expect("Error listen reply");

    let decoded_reply: (String, String, Vec<(ActorId, MultipleGameState)>) = match raw_reply {
        Ok(raw_reply) => decode(raw_reply).expect("Erroe decode reply"),
        Err(_error) => gstd::panic!("Error in getting reply"),
    };
    // println!("decoded_reply {:?}", decoded_reply);
    decoded_reply.2
}

pub async fn get_state_games_pairs(
    api: &GearApi,
    program_id: ProgramId,
    listener: &mut gclient::EventListener,
) -> Vec<(ActorId, ActorId)> {
    let request = ["Multiple".encode(), "GamesPairs".to_string().encode()].concat();

    let gas_info = api
        .calculate_handle_gas(None, program_id, request.clone(), 0, true)
        .await
        .expect("Error calculate handle gas");

    let (message_id, _) = api
        .send_message_bytes(program_id, request.clone(), gas_info.min_limit * 2, 0)
        .await
        .expect("Error send message bytes");

    let (_, raw_reply, _) = listener
        .reply_bytes_on(message_id)
        .await
        .expect("Error listen reply");

    let decoded_reply: (String, String, Vec<(ActorId, ActorId)>) = match raw_reply {
        Ok(raw_reply) => decode(raw_reply).expect("Erroe decode reply"),
        Err(_error) => gstd::panic!("Error in getting reply"),
    };
    // println!("decoded_reply {:?}", decoded_reply);
    decoded_reply.2
}

pub fn get_move_vk_proof_public() -> (InputVerifyingKeyBytes, ProofBytes, PublicMoveInput) {
    let vk_bytes = VerifyingKeyBytes {
        vk_alpha_g1: vec![
            21, 238, 187, 56, 128, 239, 100, 193, 139, 134, 182, 106, 16, 77, 185, 90, 237, 174,
            29, 23, 48, 142, 191, 93, 255, 192, 141, 46, 181, 153, 123, 119, 215, 64, 7, 212, 10,
            203, 106, 133, 176, 198, 114, 6, 183, 213, 40, 176, 0, 195, 130, 190, 223, 20, 179, 85,
            248, 89, 92, 53, 203, 106, 90, 176, 3, 244, 3, 204, 78, 22, 85, 252, 232, 121, 135,
            141, 190, 92, 123, 17, 113, 51, 207, 184, 250, 205, 178, 211, 22, 177, 253, 231, 54,
            210, 68, 185,
        ],

        vk_beta_g2: vec![
            24, 61, 186, 10, 182, 144, 76, 133, 227, 107, 128, 197, 113, 39, 49, 64, 230, 17, 71,
            171, 150, 130, 173, 122, 123, 146, 222, 25, 6, 42, 95, 71, 71, 188, 10, 64, 58, 138,
            151, 70, 202, 2, 1, 47, 49, 46, 122, 29, 15, 89, 29, 171, 202, 39, 54, 107, 210, 131,
            67, 160, 69, 62, 201, 204, 234, 174, 171, 226, 3, 65, 97, 112, 218, 64, 137, 44, 180,
            26, 3, 152, 240, 166, 184, 105, 218, 235, 162, 80, 251, 181, 136, 23, 116, 226, 129,
            212, 13, 104, 99, 202, 195, 142, 218, 65, 83, 29, 58, 193, 145, 161, 49, 150, 18, 152,
            103, 16, 58, 200, 248, 21, 35, 13, 35, 143, 202, 71, 123, 239, 68, 114, 2, 17, 38, 128,
            42, 27, 200, 202, 26, 117, 138, 226, 94, 34, 0, 150, 151, 178, 183, 51, 20, 163, 15,
            71, 246, 227, 16, 18, 180, 149, 233, 187, 115, 160, 9, 248, 190, 87, 31, 32, 164, 68,
            238, 69, 183, 161, 149, 128, 83, 136, 136, 19, 211, 211, 176, 5, 80, 142, 3, 193, 175,
            98,
        ],

        vk_gamma_g2: vec![
            19, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 79, 101, 89, 107, 208,
            208, 153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148,
            93, 87, 229, 172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5,
            39, 45, 197, 16, 81, 198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227,
            209, 119, 11, 172, 3, 38, 168, 5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184, 6, 6,
            196, 160, 46, 167, 52, 204, 50, 172, 210, 176, 43, 194, 139, 153, 203, 62, 40, 126,
            133, 167, 99, 175, 38, 116, 146, 171, 87, 46, 153, 171, 63, 55, 13, 39, 92, 236, 29,
            161, 170, 169, 7, 95, 240, 95, 121, 190, 12, 229, 213, 39, 114, 125, 110, 17, 140, 201,
            205, 198, 218, 46, 53, 26, 173, 253, 155, 170, 140, 189, 211, 167, 109, 66, 154, 105,
            81, 96, 209, 44, 146, 58, 201, 204, 59, 172, 162, 137, 225, 147, 84, 134, 8, 184, 40,
            1,
        ],

        vk_delta_g2: vec![
            21, 213, 219, 111, 105, 140, 64, 250, 21, 5, 106, 130, 22, 236, 139, 219, 31, 252, 227,
            3, 130, 203, 79, 141, 88, 105, 157, 5, 93, 37, 93, 12, 139, 104, 161, 187, 42, 40, 166,
            157, 245, 116, 239, 111, 178, 140, 134, 126, 6, 50, 94, 60, 220, 153, 90, 231, 29, 229,
            96, 73, 139, 131, 197, 164, 114, 94, 232, 148, 93, 130, 148, 123, 234, 40, 216, 7, 88,
            48, 242, 20, 217, 73, 98, 207, 108, 183, 98, 215, 193, 251, 35, 120, 71, 156, 89, 145,
            6, 155, 2, 148, 187, 248, 246, 248, 131, 26, 143, 53, 5, 12, 8, 136, 254, 213, 125,
            181, 35, 191, 236, 101, 210, 89, 104, 233, 209, 140, 112, 134, 59, 37, 88, 167, 9, 63,
            196, 15, 100, 86, 45, 240, 183, 131, 173, 115, 8, 51, 12, 146, 212, 78, 30, 145, 113,
            55, 166, 135, 117, 80, 168, 31, 123, 228, 45, 9, 57, 10, 152, 245, 146, 190, 126, 148,
            182, 56, 128, 69, 45, 126, 141, 212, 17, 211, 168, 209, 61, 177, 158, 48, 142, 149,
            208, 18,
        ],
    };

    let ic = vec![
        vec![
            11, 41, 43, 123, 59, 157, 231, 73, 234, 132, 175, 26, 250, 52, 198, 80, 51, 241, 185,
            24, 105, 63, 56, 141, 88, 124, 127, 54, 152, 211, 83, 163, 244, 11, 95, 169, 8, 239,
            192, 194, 149, 7, 244, 137, 105, 231, 242, 37, 15, 80, 116, 29, 191, 196, 169, 201,
            211, 43, 169, 35, 145, 80, 85, 253, 94, 185, 136, 138, 162, 39, 185, 181, 234, 1, 154,
            253, 216, 1, 43, 116, 179, 51, 170, 68, 112, 195, 49, 35, 18, 65, 101, 135, 255, 221,
            177, 96,
        ],
        vec![
            8, 0, 112, 195, 217, 2, 237, 252, 158, 153, 207, 107, 197, 114, 157, 6, 190, 112, 179,
            205, 43, 200, 154, 45, 151, 95, 146, 35, 124, 111, 254, 102, 236, 141, 172, 243, 46,
            14, 192, 32, 98, 50, 152, 169, 134, 131, 196, 27, 15, 109, 22, 45, 155, 226, 44, 205,
            239, 196, 215, 211, 46, 73, 30, 214, 252, 102, 154, 64, 194, 84, 132, 144, 71, 90, 48,
            113, 198, 35, 29, 130, 41, 243, 32, 194, 125, 171, 219, 68, 35, 29, 93, 140, 4, 144,
            90, 105,
        ],
        vec![
            22, 176, 60, 58, 22, 156, 56, 143, 244, 178, 157, 203, 198, 189, 179, 201, 76, 12, 178,
            212, 152, 60, 57, 58, 89, 167, 252, 188, 190, 128, 168, 232, 91, 213, 62, 207, 10, 143,
            240, 1, 46, 163, 249, 161, 151, 62, 106, 77, 18, 202, 205, 212, 179, 81, 180, 68, 185,
            78, 41, 141, 183, 148, 6, 204, 201, 157, 63, 104, 212, 4, 6, 149, 145, 243, 90, 91,
            239, 231, 250, 160, 59, 19, 10, 205, 144, 19, 140, 216, 171, 14, 43, 194, 160, 168, 73,
            201,
        ],
        vec![
            4, 245, 139, 222, 159, 175, 152, 78, 123, 39, 222, 0, 192, 185, 104, 207, 131, 231,
            151, 211, 172, 204, 187, 57, 250, 138, 249, 183, 232, 44, 231, 82, 88, 4, 181, 81, 108,
            154, 16, 125, 36, 203, 161, 87, 219, 115, 112, 6, 12, 207, 199, 0, 252, 146, 9, 255,
            177, 157, 22, 89, 109, 159, 51, 149, 10, 218, 102, 177, 15, 0, 6, 115, 183, 125, 78,
            18, 183, 84, 40, 251, 117, 141, 150, 128, 201, 87, 157, 101, 144, 125, 236, 231, 233,
            8, 145, 244,
        ],
    ];

    let alpha_g1 = G1Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_alpha_g1).unwrap();
    let beta_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_beta_g2).unwrap();
    let gamma_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_gamma_g2).unwrap();
    let delta_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_delta_g2).unwrap();

    let alpha_g1_beta_g2 = Bls12_381::pairing(alpha_g1, beta_g2).0;
    let gamma_g2_neg_pc: G2Affine = gamma_g2.into_group().neg().into_affine();
    let delta_g2_neg_pc: G2Affine = delta_g2.into_group().neg().into_affine();

    let proof_bytes = ProofBytes {
        a: vec![
            18, 62, 3, 152, 234, 187, 254, 129, 21, 92, 184, 108, 46, 89, 48, 7, 21, 244, 60, 191,
            255, 9, 46, 53, 70, 63, 111, 225, 112, 102, 107, 11, 165, 40, 77, 36, 234, 164, 51,
            106, 214, 100, 142, 137, 7, 100, 194, 145, 1, 207, 206, 207, 218, 246, 7, 64, 30, 135,
            3, 242, 113, 1, 68, 143, 242, 111, 135, 249, 251, 242, 80, 178, 194, 152, 238, 47, 59,
            203, 88, 150, 220, 89, 227, 239, 186, 18, 121, 85, 102, 231, 188, 152, 122, 36, 46, 97,
        ],
        b: vec![
            2, 30, 144, 113, 111, 53, 17, 37, 77, 98, 10, 185, 212, 0, 75, 5, 3, 238, 118, 104, 96,
            150, 196, 240, 253, 134, 59, 8, 101, 207, 194, 203, 28, 237, 189, 32, 203, 68, 71, 27,
            255, 143, 99, 136, 112, 129, 70, 32, 24, 243, 37, 244, 66, 45, 55, 38, 62, 40, 76, 115,
            28, 98, 66, 105, 131, 148, 197, 93, 62, 187, 127, 10, 41, 171, 71, 46, 78, 144, 21,
            255, 5, 118, 24, 204, 185, 15, 105, 199, 179, 18, 70, 80, 156, 36, 110, 150, 20, 208,
            231, 64, 220, 234, 214, 173, 182, 48, 187, 248, 176, 125, 32, 60, 7, 48, 143, 147, 127,
            12, 245, 250, 221, 116, 230, 42, 140, 163, 227, 156, 222, 170, 178, 65, 222, 85, 112,
            113, 78, 63, 224, 151, 116, 161, 171, 43, 10, 183, 12, 245, 120, 81, 202, 19, 54, 1,
            125, 208, 56, 88, 182, 52, 98, 94, 63, 120, 72, 43, 100, 151, 111, 176, 94, 195, 90,
            241, 235, 238, 190, 193, 205, 203, 134, 177, 147, 149, 190, 170, 219, 110, 160, 216,
            233, 166,
        ],
        c: vec![
            25, 190, 139, 182, 33, 11, 9, 222, 208, 216, 48, 3, 213, 17, 166, 171, 30, 132, 85,
            200, 122, 167, 207, 185, 142, 53, 176, 243, 163, 211, 160, 231, 225, 57, 40, 29, 195,
            226, 73, 147, 214, 78, 159, 115, 96, 108, 37, 77, 25, 253, 189, 230, 100, 166, 241,
            127, 45, 172, 211, 67, 235, 150, 49, 152, 236, 221, 227, 50, 106, 1, 20, 11, 107, 233,
            163, 196, 176, 69, 50, 126, 83, 96, 168, 90, 117, 211, 13, 13, 184, 48, 50, 138, 191,
            173, 128, 218,
        ],
    };
    let mut alpha_g1_beta_g2_bytes = Vec::new();
    alpha_g1_beta_g2
        .serialize_uncompressed(&mut alpha_g1_beta_g2_bytes)
        .unwrap();

    let mut gamma_g2_neg_pc_bytes = Vec::new();
    gamma_g2_neg_pc
        .serialize_uncompressed(&mut gamma_g2_neg_pc_bytes)
        .unwrap();

    let mut delta_g2_neg_pc_bytes = Vec::new();
    delta_g2_neg_pc
        .serialize_uncompressed(&mut delta_g2_neg_pc_bytes)
        .unwrap();

    let vk_bytes = InputVerifyingKeyBytes {
        alpha_g1_beta_g2: alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc: gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc: delta_g2_neg_pc_bytes,
        ic,
    };

    let public_input = PublicMoveInput {
        out: 1,
        hit: 1,
        hash: vec![
            40, 108, 212, 155, 236, 63, 122, 81, 224, 152, 86, 162, 255, 254, 40, 4, 190, 34, 217,
            215, 45, 69, 119, 135, 71, 38, 210, 217, 196, 31, 1, 76,
        ],
    };

    (vk_bytes, proof_bytes, public_input)
}

pub fn get_start_vk_proof_public() -> (InputVerifyingKeyBytes, ProofBytes, PublicStartInput) {
    let vk_bytes = VerifyingKeyBytes {
        vk_alpha_g1: vec![
            25, 27, 128, 228, 86, 62, 84, 152, 210, 79, 254, 225, 31, 19, 47, 56, 239, 21, 65, 140,
            73, 226, 247, 193, 220, 104, 215, 226, 35, 58, 65, 107, 132, 136, 101, 161, 3, 135, 0,
            177, 43, 21, 1, 171, 189, 126, 96, 47, 6, 215, 168, 192, 172, 86, 185, 216, 170, 122,
            135, 229, 3, 210, 239, 240, 143, 122, 206, 42, 193, 141, 2, 156, 137, 150, 151, 46, 35,
            173, 153, 224, 226, 115, 197, 171, 53, 54, 155, 128, 200, 237, 149, 193, 216, 31, 31,
            244,
        ],

        vk_beta_g2: vec![
            17, 50, 242, 249, 243, 68, 236, 251, 130, 211, 121, 71, 5, 224, 108, 37, 174, 200, 24,
            75, 124, 76, 212, 153, 200, 173, 0, 160, 29, 87, 34, 98, 207, 27, 214, 125, 120, 103,
            99, 237, 4, 17, 244, 243, 163, 29, 51, 185, 14, 29, 134, 181, 91, 54, 108, 57, 184,
            223, 6, 208, 31, 206, 117, 244, 235, 109, 143, 29, 235, 210, 126, 171, 120, 103, 220,
            13, 108, 143, 127, 249, 81, 196, 93, 15, 221, 194, 218, 225, 195, 39, 126, 78, 118,
            222, 217, 134, 2, 19, 239, 145, 251, 230, 167, 143, 27, 234, 61, 185, 188, 235, 156,
            58, 202, 236, 106, 96, 48, 245, 127, 227, 132, 170, 138, 55, 80, 31, 47, 127, 239, 18,
            16, 38, 181, 0, 26, 204, 88, 13, 194, 45, 18, 18, 7, 1, 24, 70, 179, 130, 30, 200, 80,
            213, 0, 158, 146, 184, 156, 160, 243, 39, 255, 144, 115, 127, 186, 249, 122, 52, 137,
            215, 216, 159, 65, 56, 0, 27, 49, 155, 162, 235, 176, 85, 247, 235, 86, 55, 103, 145,
            188, 134, 198, 237,
        ],

        vk_gamma_g2: vec![
            19, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 79, 101, 89, 107, 208,
            208, 153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148,
            93, 87, 229, 172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5,
            39, 45, 197, 16, 81, 198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227,
            209, 119, 11, 172, 3, 38, 168, 5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184, 6, 6,
            196, 160, 46, 167, 52, 204, 50, 172, 210, 176, 43, 194, 139, 153, 203, 62, 40, 126,
            133, 167, 99, 175, 38, 116, 146, 171, 87, 46, 153, 171, 63, 55, 13, 39, 92, 236, 29,
            161, 170, 169, 7, 95, 240, 95, 121, 190, 12, 229, 213, 39, 114, 125, 110, 17, 140, 201,
            205, 198, 218, 46, 53, 26, 173, 253, 155, 170, 140, 189, 211, 167, 109, 66, 154, 105,
            81, 96, 209, 44, 146, 58, 201, 204, 59, 172, 162, 137, 225, 147, 84, 134, 8, 184, 40,
            1,
        ],

        vk_delta_g2: vec![
            18, 61, 64, 37, 225, 154, 120, 31, 38, 164, 246, 197, 231, 46, 183, 47, 56, 72, 248,
            33, 87, 163, 181, 199, 95, 111, 229, 217, 172, 75, 154, 130, 6, 228, 216, 38, 159, 112,
            215, 180, 87, 241, 4, 32, 208, 36, 248, 175, 21, 224, 184, 102, 7, 233, 16, 91, 52,
            224, 116, 122, 125, 128, 155, 209, 81, 90, 52, 228, 107, 247, 134, 175, 37, 113, 13,
            224, 164, 28, 173, 236, 192, 210, 106, 166, 197, 63, 137, 254, 82, 225, 213, 41, 213,
            212, 92, 7, 4, 208, 236, 121, 54, 15, 50, 5, 144, 226, 232, 99, 145, 166, 222, 74, 78,
            230, 33, 13, 38, 107, 127, 107, 176, 63, 240, 28, 166, 231, 101, 213, 133, 128, 37,
            181, 2, 135, 205, 50, 223, 83, 210, 22, 53, 215, 112, 194, 23, 15, 208, 133, 98, 216,
            82, 22, 216, 114, 106, 162, 163, 228, 91, 204, 194, 235, 2, 81, 231, 255, 254, 31, 172,
            182, 182, 184, 216, 202, 173, 248, 244, 224, 181, 109, 112, 65, 19, 29, 87, 37, 104,
            181, 68, 185, 45, 72,
        ],
    };

    let ic = vec![
        vec![
            10, 44, 27, 93, 157, 157, 41, 205, 215, 154, 51, 251, 179, 241, 105, 220, 190, 59, 6,
            130, 155, 205, 146, 71, 82, 215, 125, 114, 29, 31, 52, 150, 242, 4, 16, 223, 197, 147,
            19, 31, 123, 218, 154, 97, 228, 85, 20, 206, 8, 71, 147, 234, 28, 46, 223, 29, 27, 83,
            60, 17, 17, 75, 15, 25, 56, 133, 242, 17, 216, 54, 169, 164, 117, 220, 150, 48, 252,
            100, 117, 213, 5, 45, 137, 84, 231, 190, 188, 188, 6, 202, 219, 238, 32, 59, 19, 85,
        ],
        vec![
            7, 56, 196, 104, 229, 198, 223, 8, 201, 170, 153, 184, 153, 170, 174, 209, 173, 39, 54,
            140, 217, 163, 246, 73, 54, 218, 166, 37, 193, 68, 211, 235, 47, 244, 163, 130, 99,
            132, 180, 111, 235, 152, 191, 139, 166, 116, 168, 228, 15, 59, 182, 222, 185, 121, 57,
            238, 98, 150, 90, 108, 143, 173, 90, 126, 169, 160, 225, 135, 71, 7, 236, 195, 251, 15,
            0, 64, 84, 179, 140, 141, 48, 67, 199, 97, 101, 112, 53, 228, 187, 40, 39, 1, 130, 5,
            121, 193,
        ],
    ];

    let alpha_g1 = G1Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_alpha_g1).unwrap();
    let beta_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_beta_g2).unwrap();
    let gamma_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_gamma_g2).unwrap();
    let delta_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_delta_g2).unwrap();

    let alpha_g1_beta_g2 = Bls12_381::pairing(alpha_g1, beta_g2).0;
    let gamma_g2_neg_pc: G2Affine = gamma_g2.into_group().neg().into_affine();
    let delta_g2_neg_pc: G2Affine = delta_g2.into_group().neg().into_affine();

    let proof_bytes = ProofBytes {
        a: vec![
            15, 111, 194, 110, 167, 20, 233, 209, 28, 162, 2, 108, 110, 83, 34, 145, 153, 82, 124,
            58, 130, 6, 97, 37, 91, 248, 207, 116, 215, 90, 255, 137, 134, 241, 98, 80, 231, 202,
            67, 50, 27, 92, 225, 106, 16, 105, 184, 88, 14, 196, 110, 50, 33, 176, 50, 140, 6, 185,
            101, 29, 48, 187, 118, 217, 225, 48, 102, 174, 60, 253, 48, 49, 214, 89, 42, 116, 217,
            224, 228, 118, 208, 157, 210, 186, 156, 53, 210, 62, 243, 17, 15, 253, 159, 26, 21, 52,
        ],
        b: vec![
            15, 248, 131, 136, 91, 213, 216, 17, 67, 222, 103, 20, 1, 36, 48, 234, 184, 151, 76,
            25, 181, 240, 160, 105, 43, 215, 20, 108, 197, 234, 87, 173, 249, 102, 157, 22, 157,
            106, 119, 163, 28, 46, 194, 229, 172, 231, 91, 73, 16, 113, 144, 191, 72, 213, 95, 120,
            104, 143, 111, 250, 219, 180, 37, 63, 128, 171, 99, 82, 98, 21, 101, 126, 225, 102, 64,
            148, 161, 125, 98, 156, 238, 88, 204, 138, 107, 77, 143, 179, 7, 100, 24, 65, 26, 223,
            20, 90, 0, 63, 144, 80, 190, 139, 165, 128, 8, 231, 240, 46, 146, 17, 254, 120, 127,
            46, 46, 113, 208, 12, 22, 162, 98, 208, 253, 49, 53, 138, 55, 222, 35, 49, 107, 224, 5,
            230, 254, 51, 136, 131, 89, 162, 166, 177, 177, 250, 13, 80, 181, 191, 219, 80, 124,
            37, 132, 116, 172, 190, 67, 244, 126, 104, 254, 86, 67, 32, 2, 25, 29, 96, 235, 118,
            144, 209, 215, 93, 221, 72, 186, 214, 177, 19, 132, 101, 122, 131, 132, 16, 86, 94, 87,
            49, 59, 3,
        ],
        c: vec![
            6, 205, 235, 22, 219, 249, 160, 246, 210, 224, 98, 42, 106, 181, 56, 8, 43, 237, 16,
            43, 215, 35, 65, 0, 145, 26, 11, 81, 0, 197, 112, 5, 77, 207, 67, 41, 194, 239, 239,
            27, 90, 8, 105, 126, 144, 76, 57, 236, 2, 52, 115, 196, 143, 172, 2, 181, 6, 114, 133,
            245, 79, 138, 11, 86, 78, 95, 110, 114, 78, 247, 33, 196, 236, 151, 39, 156, 194, 143,
            48, 93, 80, 80, 179, 122, 212, 232, 150, 112, 118, 5, 232, 206, 212, 194, 82, 80,
        ],
    };
    let mut alpha_g1_beta_g2_bytes = Vec::new();
    alpha_g1_beta_g2
        .serialize_uncompressed(&mut alpha_g1_beta_g2_bytes)
        .unwrap();

    let mut gamma_g2_neg_pc_bytes = Vec::new();
    gamma_g2_neg_pc
        .serialize_uncompressed(&mut gamma_g2_neg_pc_bytes)
        .unwrap();

    let mut delta_g2_neg_pc_bytes = Vec::new();
    delta_g2_neg_pc
        .serialize_uncompressed(&mut delta_g2_neg_pc_bytes)
        .unwrap();

    let vk_bytes = InputVerifyingKeyBytes {
        alpha_g1_beta_g2: alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc: gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc: delta_g2_neg_pc_bytes,
        ic,
    };

    let public_input = PublicStartInput {
        hash: vec![
            51, 217, 233, 61, 233, 121, 204, 172, 68, 169, 118, 202, 251, 95, 229, 50, 34, 187, 67,
            43, 194, 51, 134, 75, 59, 97, 49, 24, 246, 190, 33, 18,
        ],
    };

    (vk_bytes, proof_bytes, public_input)
}

pub async fn get_new_client(api: &GearApi, name: &str) -> GearApi {
    let alice_balance = api
        .total_balance(api.account_id())
        .await
        .expect("Error total balance");
    let amount = alice_balance / 5;
    api.transfer_keep_alive(
        api.get_specific_actor_id(name)
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        amount,
    )
    .await
    .expect("Error transfer");

    api.clone().with(name).expect("Unable to change signer.")
}

pub fn get_test_move_vk_proof_public() -> (InputVerifyingKeyBytes, ProofBytes, PublicMoveInput) {
    let vk_bytes = VerifyingKeyBytes {
        vk_alpha_g1: vec![
            11, 246, 175, 229, 25, 140, 184, 122, 88, 181, 157, 6, 89, 3, 16, 248, 81, 117, 109,
            55, 4, 205, 45, 77, 101, 91, 43, 155, 27, 8, 245, 179, 168, 251, 65, 199, 198, 210,
            122, 165, 205, 176, 82, 251, 138, 38, 52, 82, 2, 14, 194, 146, 170, 173, 124, 108, 194,
            77, 216, 208, 209, 234, 7, 22, 123, 61, 231, 194, 43, 219, 255, 40, 176, 2, 122, 23,
            68, 160, 35, 150, 202, 174, 155, 248, 3, 215, 243, 215, 4, 158, 197, 131, 52, 164, 147,
            113,
        ],

        vk_beta_g2: vec![
            24, 56, 165, 234, 102, 160, 187, 90, 241, 46, 255, 199, 90, 38, 142, 184, 219, 44, 197,
            148, 95, 120, 143, 108, 232, 148, 244, 56, 118, 3, 134, 68, 99, 36, 78, 24, 119, 68,
            108, 64, 2, 112, 31, 234, 94, 106, 20, 39, 12, 102, 245, 80, 204, 155, 102, 29, 225,
            155, 245, 40, 56, 47, 188, 213, 195, 160, 171, 13, 60, 175, 21, 122, 227, 77, 195, 14,
            25, 235, 241, 53, 65, 66, 47, 90, 47, 84, 115, 218, 62, 41, 45, 11, 15, 161, 237, 25,
            21, 18, 152, 202, 86, 194, 13, 95, 215, 54, 126, 109, 127, 24, 110, 234, 71, 171, 76,
            72, 180, 222, 16, 143, 80, 82, 148, 21, 225, 217, 118, 233, 147, 239, 240, 136, 150,
            219, 91, 4, 232, 19, 250, 79, 162, 229, 199, 126, 24, 224, 49, 22, 251, 182, 35, 96,
            168, 65, 96, 157, 175, 199, 245, 42, 215, 30, 7, 47, 223, 85, 251, 15, 208, 69, 192,
            20, 161, 120, 177, 160, 169, 7, 104, 211, 150, 142, 42, 118, 80, 157, 209, 46, 251, 99,
            23, 229,
        ],

        vk_gamma_g2: vec![
            19, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 79, 101, 89, 107, 208,
            208, 153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148,
            93, 87, 229, 172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5,
            39, 45, 197, 16, 81, 198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227,
            209, 119, 11, 172, 3, 38, 168, 5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184, 6, 6,
            196, 160, 46, 167, 52, 204, 50, 172, 210, 176, 43, 194, 139, 153, 203, 62, 40, 126,
            133, 167, 99, 175, 38, 116, 146, 171, 87, 46, 153, 171, 63, 55, 13, 39, 92, 236, 29,
            161, 170, 169, 7, 95, 240, 95, 121, 190, 12, 229, 213, 39, 114, 125, 110, 17, 140, 201,
            205, 198, 218, 46, 53, 26, 173, 253, 155, 170, 140, 189, 211, 167, 109, 66, 154, 105,
            81, 96, 209, 44, 146, 58, 201, 204, 59, 172, 162, 137, 225, 147, 84, 134, 8, 184, 40,
            1,
        ],

        vk_delta_g2: vec![
            20, 209, 76, 38, 165, 53, 219, 247, 74, 4, 222, 60, 143, 185, 208, 251, 244, 190, 237,
            216, 141, 192, 219, 131, 108, 237, 179, 210, 196, 112, 40, 140, 206, 26, 216, 86, 45,
            144, 211, 205, 147, 59, 103, 199, 93, 86, 141, 82, 22, 233, 153, 120, 211, 179, 250,
            184, 45, 17, 204, 111, 253, 26, 224, 149, 52, 127, 1, 96, 156, 117, 102, 3, 65, 195,
            149, 190, 106, 16, 15, 205, 40, 84, 77, 152, 43, 151, 163, 216, 115, 214, 184, 6, 167,
            221, 254, 94, 4, 174, 185, 88, 29, 135, 230, 105, 117, 58, 73, 118, 87, 80, 70, 143,
            79, 238, 254, 229, 1, 129, 54, 206, 27, 255, 75, 24, 158, 73, 230, 175, 99, 221, 29,
            161, 50, 158, 63, 31, 239, 48, 139, 124, 140, 58, 173, 158, 4, 101, 8, 48, 121, 36,
            196, 18, 252, 68, 162, 74, 22, 78, 120, 37, 71, 46, 99, 1, 228, 177, 94, 114, 116, 19,
            202, 230, 22, 194, 0, 104, 237, 48, 25, 135, 124, 166, 8, 168, 161, 155, 175, 60, 157,
            254, 233, 147,
        ],
    };

    let ic = vec![
        vec![
            12, 145, 248, 226, 253, 142, 132, 49, 66, 68, 247, 180, 87, 254, 50, 200, 168, 18, 160,
            105, 189, 201, 170, 154, 101, 182, 173, 157, 0, 146, 97, 134, 47, 142, 74, 146, 50,
            164, 254, 167, 162, 157, 111, 149, 168, 187, 173, 208, 8, 199, 67, 229, 179, 1, 96,
            164, 105, 253, 30, 245, 255, 197, 252, 250, 246, 227, 141, 7, 231, 136, 123, 197, 145,
            237, 90, 49, 135, 148, 83, 87, 89, 176, 146, 221, 114, 242, 77, 42, 31, 122, 215, 76,
            95, 111, 86, 66,
        ],
        vec![
            6, 31, 176, 51, 11, 82, 63, 91, 101, 254, 27, 141, 154, 172, 183, 79, 216, 248, 86,
            196, 207, 172, 216, 92, 142, 69, 202, 205, 145, 46, 215, 4, 166, 248, 251, 233, 133,
            207, 89, 142, 16, 126, 220, 220, 11, 34, 144, 197, 7, 189, 179, 217, 237, 172, 33, 105,
            154, 20, 179, 71, 174, 124, 103, 91, 255, 205, 230, 253, 25, 203, 68, 84, 123, 92, 210,
            247, 65, 23, 228, 198, 6, 131, 72, 173, 227, 195, 142, 79, 101, 13, 202, 201, 147, 101,
            220, 236,
        ],
        // vec![
        // 	1,220,106,233,82,7,6,81,60,180,23,212,68,13,95,201,249,242,217,26,251,47,164,190,132,37,202,196,223,219,179,64,14,118,88,107,57,157,136,167,52,23,143,242,5,29,125,14,
        // 	9,88,229,181,14,205,91,188,249,87,10,108,235,32,167,203,61,243,221,143,201,15,153,49,77,213,138,40,53,141,22,145,227,135,10,248,45,218,124,82,13,117,28,54,113,3,107,200,
        // ],
        // vec![
        // 	14,229,203,38,163,118,154,195,109,163,159,35,134,155,109,199,178,128,49,197,187,68,65,20,211,175,39,144,65,7,168,143,247,31,13,83,77,219,41,29,31,206,152,78,232,117,69,213,
        // 	11,17,98,18,77,121,27,1,56,36,187,90,5,216,134,40,89,63,164,79,148,154,202,200,243,39,83,225,250,213,1,107,224,137,228,65,67,91,8,178,31,176,233,55,42,234,161,107,
        // ],
    ];

    let alpha_g1 = G1Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_alpha_g1).unwrap();
    let beta_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_beta_g2).unwrap();
    let gamma_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_gamma_g2).unwrap();
    let delta_g2 = G2Affine::deserialize_uncompressed_unchecked(&*vk_bytes.vk_delta_g2).unwrap();

    let alpha_g1_beta_g2 = Bls12_381::pairing(alpha_g1, beta_g2).0;
    let gamma_g2_neg_pc: G2Affine = gamma_g2.into_group().neg().into_affine();
    let delta_g2_neg_pc: G2Affine = delta_g2.into_group().neg().into_affine();

    let proof_bytes = ProofBytes {
        a: vec![
            18, 62, 3, 152, 234, 187, 254, 129, 21, 92, 184, 108, 46, 89, 48, 7, 21, 244, 60, 191,
            255, 9, 46, 53, 70, 63, 111, 225, 112, 102, 107, 11, 165, 40, 77, 36, 234, 164, 51,
            106, 214, 100, 142, 137, 7, 100, 194, 145, 1, 207, 206, 207, 218, 246, 7, 64, 30, 135,
            3, 242, 113, 1, 68, 143, 242, 111, 135, 249, 251, 242, 80, 178, 194, 152, 238, 47, 59,
            203, 88, 150, 220, 89, 227, 239, 186, 18, 121, 85, 102, 231, 188, 152, 122, 36, 46, 97,
        ],
        b: vec![
            2, 30, 144, 113, 111, 53, 17, 37, 77, 98, 10, 185, 212, 0, 75, 5, 3, 238, 118, 104, 96,
            150, 196, 240, 253, 134, 59, 8, 101, 207, 194, 203, 28, 237, 189, 32, 203, 68, 71, 27,
            255, 143, 99, 136, 112, 129, 70, 32, 24, 243, 37, 244, 66, 45, 55, 38, 62, 40, 76, 115,
            28, 98, 66, 105, 131, 148, 197, 93, 62, 187, 127, 10, 41, 171, 71, 46, 78, 144, 21,
            255, 5, 118, 24, 204, 185, 15, 105, 199, 179, 18, 70, 80, 156, 36, 110, 150, 20, 208,
            231, 64, 220, 234, 214, 173, 182, 48, 187, 248, 176, 125, 32, 60, 7, 48, 143, 147, 127,
            12, 245, 250, 221, 116, 230, 42, 140, 163, 227, 156, 222, 170, 178, 65, 222, 85, 112,
            113, 78, 63, 224, 151, 116, 161, 171, 43, 10, 183, 12, 245, 120, 81, 202, 19, 54, 1,
            125, 208, 56, 88, 182, 52, 98, 94, 63, 120, 72, 43, 100, 151, 111, 176, 94, 195, 90,
            241, 235, 238, 190, 193, 205, 203, 134, 177, 147, 149, 190, 170, 219, 110, 160, 216,
            233, 166,
        ],
        c: vec![
            25, 190, 139, 182, 33, 11, 9, 222, 208, 216, 48, 3, 213, 17, 166, 171, 30, 132, 85,
            200, 122, 167, 207, 185, 142, 53, 176, 243, 163, 211, 160, 231, 225, 57, 40, 29, 195,
            226, 73, 147, 214, 78, 159, 115, 96, 108, 37, 77, 25, 253, 189, 230, 100, 166, 241,
            127, 45, 172, 211, 67, 235, 150, 49, 152, 236, 221, 227, 50, 106, 1, 20, 11, 107, 233,
            163, 196, 176, 69, 50, 126, 83, 96, 168, 90, 117, 211, 13, 13, 184, 48, 50, 138, 191,
            173, 128, 218,
        ],
    };
    let mut alpha_g1_beta_g2_bytes = Vec::new();
    alpha_g1_beta_g2
        .serialize_uncompressed(&mut alpha_g1_beta_g2_bytes)
        .unwrap();

    let mut gamma_g2_neg_pc_bytes = Vec::new();
    gamma_g2_neg_pc
        .serialize_uncompressed(&mut gamma_g2_neg_pc_bytes)
        .unwrap();

    let mut delta_g2_neg_pc_bytes = Vec::new();
    delta_g2_neg_pc
        .serialize_uncompressed(&mut delta_g2_neg_pc_bytes)
        .unwrap();

    let vk_bytes = InputVerifyingKeyBytes {
        alpha_g1_beta_g2: alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc: gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc: delta_g2_neg_pc_bytes,
        ic,
    };

    let public_input = PublicMoveInput {
        out: 1,
        hit: 1,
        hash: vec![
            40, 108, 212, 155, 236, 63, 122, 81, 224, 152, 86, 162, 255, 254, 40, 4, 190, 34, 217,
            215, 45, 69, 119, 135, 71, 38, 210, 217, 196, 31, 1, 76,
        ],
    };

    (vk_bytes, proof_bytes, public_input)
}
