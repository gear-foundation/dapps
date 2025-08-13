#![allow(static_mut_refs)]
use sails_rs::prelude::*;
use session_service::*;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ActionsForSession {
    AllActions,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct SignatureInfo {
    pub signature_data: SignatureData,
    pub signature: Option<Vec<u8>>,
}

generate_session_system!(ActionsForSession);

impl SessionService {
    pub fn create_session_for_admin(
        signature_data: SignatureData,
        signature: Option<Vec<u8>>,
        admin: ActorId,
    ) -> Self {
        let sessions = unsafe { STORAGE.as_mut().expect("Storage is not initialized") };
        let config = unsafe { CONFIG.as_ref().expect("Config is not initialized") };
        if signature_data.duration < config.minimum_session_duration_ms {
            panic!("Duration is small");
        }

        let block_timestamp = exec::block_timestamp();
        let block_height = exec::block_height();

        let expires = block_timestamp + signature_data.duration;

        let number_of_blocks = u32::try_from(signature_data.duration.div_ceil(config.ms_per_block))
            .expect("Duration is too large");

        if signature_data.allowed_actions.is_empty() {
            panic!("There are no allowed messages");
        }

        let account = match signature {
            Some(sig_bytes) => {
                check_if_session_exists(sessions, &signature_data.key)
                    .expect("Error check session exists");
                let pub_key: [u8; 32] = (signature_data.key).into();
                let message = SignatureData {
                    key: admin,
                    duration: signature_data.duration,
                    allowed_actions: signature_data.allowed_actions.clone(),
                }
                .encode();

                let complete_message =
                    [b"<Bytes>".to_vec(), message, b"</Bytes>".to_vec()].concat();

                verify(&sig_bytes, complete_message, pub_key).expect("Error verify");
                sessions.entry(signature_data.key).insert(SessionData {
                    key: admin,
                    expires,
                    allowed_actions: signature_data.allowed_actions,
                    expires_at_block: block_height + number_of_blocks,
                });
                signature_data.key
            }
            None => {
                check_if_session_exists(sessions, &admin).expect("Error check session exists");
                sessions.entry(admin).insert(SessionData {
                    key: signature_data.key,
                    expires,
                    allowed_actions: signature_data.allowed_actions,
                    expires_at_block: block_height + number_of_blocks,
                });
                admin
            }
        };

        let request = [
            "Session".encode(),
            "DeleteSessionFromProgram".to_string().encode(),
            (account).encode(),
        ]
        .concat();

        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            request,
            config.gas_to_delete_session,
            0,
            number_of_blocks,
        )
        .expect("Error in sending message");
        Self(())
    }
}
