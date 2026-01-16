use super::{
    sr25519::verify,
    utils::{Result, *},
};
use crate::admin::storage::configuration::Configuration;
use gstd::{ActorId, Encode, exec, msg, prelude::*};

pub fn create_session(
    session_map: &mut SessionMap,
    config: Configuration,
    signature_data: SignatureData,
    signature: Option<Vec<u8>>,
) -> Result<bool> {
    if signature_data.duration < config.minimum_session_duration_ms {
        return Err(Error::DurationIsSmall);
    }
    let source = msg::source();
    let block_timestamp = exec::block_timestamp();
    let block_height = exec::block_height();

    let expires = block_timestamp + signature_data.duration;

    let number_of_blocks =
        u32::try_from(signature_data.duration.div_ceil(config.block_duration_ms))
            .expect("Duration is too large");

    if signature_data.allowed_actions.is_empty() {
        return Err(Error::AllowedActionsIsEmpty);
    }
    let account = match signature {
        Some(sig_bytes) => {
            check_if_session_exists(session_map, &signature_data.key, block_height)?;
            let pub_key: [u8; 32] = (signature_data.key).into();
            let mut prefix = b"<Bytes>".to_vec();
            let mut message = SignatureData {
                key: source,
                duration: signature_data.duration,
                allowed_actions: signature_data.allowed_actions.clone(),
            }
            .encode();

            let mut postfix = b"</Bytes>".to_vec();
            prefix.append(&mut message);
            prefix.append(&mut postfix);

            verify(&sig_bytes, prefix, pub_key)?;
            session_map.entry(signature_data.key).insert(Session {
                key: source,
                expires,
                allowed_actions: signature_data.allowed_actions,
                expires_at_block: block_height + number_of_blocks,
            });
            signature_data.key
        }
        None => {
            check_if_session_exists(session_map, &source, block_height)?;

            session_map.entry(source).insert(Session {
                key: signature_data.key,
                expires,
                allowed_actions: signature_data.allowed_actions,
                expires_at_block: block_height + number_of_blocks,
            });
            source
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
        config.gas_for_delete_session,
        0,
        number_of_blocks,
    )
    .expect("Error in sending message");

    Ok(true)
}

fn check_if_session_exists(
    session_map: &SessionMap,
    account: &ActorId,
    block_height: u32,
) -> Result<(), Error> {
    if let Some(Session {
        key: _,
        expires: _,
        allowed_actions: _,
        expires_at_block,
    }) = session_map.get(account)
        && *expires_at_block > block_height
    {
        return Err(Error::AlreadyHaveActiveSession);
    };

    Ok(())
}

pub fn delete_session_from_program(
    session_map: &mut SessionMap,
    session_for_account: ActorId,
) -> Result<()> {
    if exec::program_id() != msg::source() {
        return Err(Error::AccessDenied);
    }
    if let Some(session) = session_map.remove(&session_for_account)
        && session.expires_at_block > exec::block_height()
    {
        return Err(Error::AccessDenied);
    }

    Ok(())
}

pub fn delete_session_from_account(session_map: &mut SessionMap, source: ActorId) -> Result<()> {
    session_map.remove(&source);
    Ok(())
}

pub fn get_player(
    session_map: &SessionMap,
    source: ActorId,
    session_for_account: &Option<ActorId>,
    actions_for_session: ActionsForSession,
) -> ActorId {
    match session_for_account {
        Some(account) => {
            let session = session_map
                .get(account)
                .expect("This account has no valid session");
            assert!(
                session.expires > exec::block_timestamp(),
                "The session has already expired"
            );
            assert!(
                session.allowed_actions.contains(&actions_for_session),
                "This message is not allowed"
            );
            assert_eq!(
                session.key, source,
                "The account is not approved for this session"
            );
            *account
        }
        None => source,
    }
}
