use crate::services::{
    game::utils::Config,
    session::{Event, SessionData, SessionError, SessionMap, SignatureData},
};
use gstd::{exec, msg};
use sails_rs::{collections::HashMap, prelude::*};

use schnorrkel::PublicKey;

pub fn create_session(
    sessions: &mut SessionMap,
    config: &Config,
    signature_data: SignatureData,
    signature: Option<Vec<u8>>,
) -> Result<Event, SessionError> {
    if signature_data.duration < config.minimum_session_duration_ms {
        return Err(SessionError::DurationIsSmall);
    }

    let msg_source = msg::source();
    let block_timestamp = exec::block_timestamp();
    let block_height = exec::block_height();

    let expires = block_timestamp + signature_data.duration;

    let number_of_blocks =
        u32::try_from(signature_data.duration.div_ceil(config.s_per_block * 1_000))
            .expect("Duration is too large");

    if signature_data.allowed_actions.is_empty() {
        return Err(SessionError::ThereAreNoAllowedMessages);
    }

    let account = match signature {
        Some(sig_bytes) => {
            check_if_session_exists(sessions, &signature_data.key)?;
            let pub_key: [u8; 32] = (signature_data.key).into();
            let mut prefix = b"<Bytes>".to_vec();
            let mut message = SignatureData {
                key: msg_source,
                duration: signature_data.duration,
                allowed_actions: signature_data.allowed_actions.clone(),
            }
            .encode();
            let mut postfix = b"</Bytes>".to_vec();
            prefix.append(&mut message);
            prefix.append(&mut postfix);

            verify(&sig_bytes, prefix, pub_key)?;
            sessions.entry(signature_data.key).insert(SessionData {
                key: msg_source,
                expires,
                allowed_actions: signature_data.allowed_actions,
                expires_at_block: block_height + number_of_blocks,
            });
            signature_data.key
        }
        None => {
            check_if_session_exists(sessions, &msg_source)?;

            sessions.entry(msg_source).insert(SessionData {
                key: signature_data.key,
                expires,
                allowed_actions: signature_data.allowed_actions,
                expires_at_block: block_height + number_of_blocks,
            });
            msg_source
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

    Ok(Event::SessionCreated)
}

pub fn delete_session_from_program(
    sessions: &mut SessionMap,
    session_for_account: ActorId,
) -> Result<Event, SessionError> {
    if msg::source() != exec::program_id() {
        return Err(SessionError::MessageOnlyForProgram);
    }

    if let Some(session) = sessions.remove(&session_for_account)
        && session.expires_at_block > exec::block_height()
    {
        return Err(SessionError::TooEarlyToDeleteSession);
    }

    Ok(Event::SessionDeleted)
}

pub fn delete_session_from_account(sessions: &mut SessionMap) -> Result<Event, SessionError> {
    if sessions.remove(&msg::source()).is_none() {
        return Err(SessionError::NoSession);
    }
    Ok(Event::SessionDeleted)
}

fn verify<P: AsRef<[u8]>, M: AsRef<[u8]>>(
    signature: &[u8],
    message: M,
    pubkey: P,
) -> Result<(), SessionError> {
    let signature =
        schnorrkel::Signature::from_bytes(signature).map_err(|_| SessionError::BadSignature)?;
    let pub_key = PublicKey::from_bytes(pubkey.as_ref()).map_err(|_| SessionError::BadPublicKey)?;
    pub_key
        .verify_simple(b"substrate", message.as_ref(), &signature)
        .map(|_| ())
        .map_err(|_| SessionError::VerificationFailed)
}

fn check_if_session_exists(
    session_map: &HashMap<ActorId, SessionData>,
    account: &ActorId,
) -> Result<(), SessionError> {
    if let Some(SessionData {
        key: _,
        expires: _,
        allowed_actions: _,
        expires_at_block,
    }) = session_map.get(account)
        && *expires_at_block > exec::block_height()
    {
        return Err(SessionError::AlreadyHaveActiveSession);
    }

    Ok(())
}
