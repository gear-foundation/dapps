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

impl<'a> SessionService<'a> {
    pub fn create_session_for_admin(
        &mut self,
        signature_data: SignatureData,
        signature: Option<Vec<u8>>,
        admin: ActorId,
    ) -> Result<(), SessionError> {
        let mut storage = self.get_mut();

        if signature_data.duration < storage.config.minimum_session_duration_ms {
            return Err(SessionError::DurationIsSmall);
        }

        let block_timestamp = exec::block_timestamp();
        let block_height = exec::block_height();
        let expires = block_timestamp + signature_data.duration;

        let number_of_blocks = u32::try_from(
            signature_data
                .duration
                .div_ceil(storage.config.ms_per_block),
        )
        .map_err(|_| SessionError::DurationIsLarge)?;

        if signature_data.allowed_actions.is_empty() {
            return Err(SessionError::ThereAreNoAllowedMessages);
        }

        let account = match signature {
            Some(sig_bytes) => {
                check_if_session_exists(&storage.sessions, &signature_data.key)?;

                let pub_key: [u8; 32] = signature_data.key.into();

                let message = SignatureData {
                    key: admin,
                    duration: signature_data.duration,
                    allowed_actions: signature_data.allowed_actions.clone(),
                }
                .encode();

                let mut complete_message =
                    Vec::with_capacity(b"<Bytes>".len() + message.len() + b"</Bytes>".len());
                complete_message.extend_from_slice(b"<Bytes>");
                complete_message.extend_from_slice(&message);
                complete_message.extend_from_slice(b"</Bytes>");

                verify(&sig_bytes, complete_message, pub_key)?;

                storage.sessions.insert(
                    signature_data.key,
                    SessionData {
                        key: admin,
                        expires,
                        allowed_actions: signature_data.allowed_actions,
                        expires_at_block: block_height + number_of_blocks,
                    },
                );

                signature_data.key
            }
            None => {
                check_if_session_exists(&storage.sessions, &admin)?;

                storage.sessions.insert(
                    admin,
                    SessionData {
                        key: signature_data.key,
                        expires,
                        allowed_actions: signature_data.allowed_actions,
                        expires_at_block: block_height + number_of_blocks,
                    },
                );

                admin
            }
        };

        let request = [
            "Session".encode(),
            "DeleteSessionFromProgram".to_string().encode(),
            account.encode(),
        ]
        .concat();

        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            request,
            storage.config.gas_to_delete_session,
            0,
            number_of_blocks,
        )
        .map_err(|_| SessionError::SendMessageFailed)?;

        Ok(())
    }
}
