use gclient::{EventProcessor, GearApi, Result};
use gear_core::ids::{MessageId, ProgramId};
use gstd::{ActorId, Decode, Encode};

use ark_std::ops::Neg;
use gbuiltin_bls381::ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use gbuiltin_bls381::ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};
use gbuiltin_bls381::ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use hex_literal::hex;
use zk_battleship::services::single::verify::VerifyingKeyBytes as InputVerifyingKeyBytes;
use zk_battleship::services::single::verify::{ProofBytes, PublicInput};
use zk_battleship::services::single::SingleGameState;

const BUILTIN_BLS381: ActorId = ActorId::new(hex!(
    "6b6e292c382945e80bf51af2ba7fe9f458dcff81ae6075c46f9095e1bbecdc37"
));

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
    (api: $api: expr, program_id: $program_id: expr, service_name: $name: literal, action: $action: literal, payload: ($($val: expr),*)) => {
        {
            let request = [
                $name.encode(),
                $action.to_string().encode(),
                ( $( $val, )*).encode(),
            ]
            .concat();

            let gas_info = $api
                .calculate_handle_gas(None, $program_id, request.clone(), 0, true)
                .await?;

            let (message_id, _) = $api
                .send_message_bytes($program_id, request.clone(), gas_info.min_limit*2, 0)
                .await?;

            message_id
        }

    };
}

pub async fn init(api: &GearApi) -> (MessageId, ProgramId) {
    let request = ["New".encode(), BUILTIN_BLS381.encode()].concat();
    let path = "../../target/wasm32-unknown-unknown/release/zk_battleship_wasm.opt.wasm";
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

pub async fn get_state_games(
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

pub fn get_vk_proof_public_ic() -> (
    InputVerifyingKeyBytes,
    ProofBytes,
    PublicInput,
    [Vec<u8>; 4],
) {
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

    let ic: [Vec<u8>; 4] = [
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
    let gamma_g2_neg_pc: G2Affine = gamma_g2.into_group().neg().into_affine().into();
    let delta_g2_neg_pc: G2Affine = delta_g2.into_group().neg().into_affine().into();

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
    };

    let public_input = PublicInput {
        out: 1,
        hit: 1,
        hash: vec![
            40, 108, 212, 155, 236, 63, 122, 81, 224, 152, 86, 162, 255, 254, 40, 4, 190, 34, 217,
            215, 45, 69, 119, 135, 71, 38, 210, 217, 196, 31, 1, 76,
        ],
    };

    (vk_bytes, proof_bytes, public_input, ic)
}
