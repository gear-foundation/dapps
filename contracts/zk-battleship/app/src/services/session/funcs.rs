use super::utils::{Result, *};
use gstd::{exec, prelude::*, ActorId};

pub fn create_session(
    session_map: &mut SessionMap,
    source: ActorId,
    block_timestamp: u64,
    key: ActorId,
    duration: u64,
    allowed_actions: Vec<ActionsForSession>,
) -> Result<bool> {
    if allowed_actions.is_empty() {
        return Err(Error::AllowedActionsIsEmpty);
    }
    if let Some(Session {
        key: _,
        expires,
        allowed_actions: _,
    }) = session_map.get(&source)
    {
        if *expires > block_timestamp {
            return Err(Error::AlreadyHaveActiveSession);
        }
    }
    session_map.entry(source).or_insert_with(|| Session {
        key,
        expires: block_timestamp + duration,
        allowed_actions,
    });
    Ok(true)
}

pub fn delete_session(session_map: &mut SessionMap, source: ActorId) -> Result<()> {
    if session_map.remove(&source).is_none() {
        return Err(Error::NoActiveSession);
    }
    Ok(())
}

pub fn get_player(
    session_map: &SessionMap,
    source: ActorId,
    session_for_account: &Option<ActorId>,
    actions_for_session: ActionsForSession,
) -> ActorId {
    let player = match session_for_account {
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
    };
    player
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::session::funcs;
    use utils::*;
    #[test]
    fn create_session() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating empty sessions map.
        let mut sessions_map = sessions_map([]);
        assert!(sessions_map.is_empty());

        let source = alice();
        let key: ActorId = 1.into();
        let duration = 100;
        let allowed_actions = vec![ActionsForSession::PlaySingleGame];

        // # Test case #1.
        // Ok: Create session
        {
            funcs::create_session(
                &mut sessions_map,
                source,
                0,
                key,
                duration,
                allowed_actions.clone(),
            )
            .unwrap();
            assert_eq!(
                *sessions_map.get(&source).unwrap(),
                Session {
                    key,
                    expires: duration,
                    allowed_actions: allowed_actions.clone()
                }
            );
        }
        // # Test case #2.
        // Error: Allowed actions is empty
        {
            let res = funcs::create_session(&mut sessions_map, source, 0, key, duration, vec![]);
            assert!(res.is_err_and(|err| err == Error::AllowedActionsIsEmpty));
        }

        // # Test case #3.
        // Error: Already have active session
        {
            let res = funcs::create_session(
                &mut sessions_map,
                source,
                0,
                key,
                duration,
                allowed_actions.clone(),
            );
            assert!(res.is_err_and(|err| err == Error::AlreadyHaveActiveSession));
        }
    }

    #[test]
    fn delete_session() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating session map.
        let source = alice();
        let session = Session {
            key: 1.into(),
            expires: 100,
            allowed_actions: vec![ActionsForSession::PlaySingleGame],
        };
        let mut sessions_map = sessions_map([(source, session)]);
        assert!(!sessions_map.is_empty());

        // # Test case #1.
        // Ok: delete session
        {
            funcs::delete_session(&mut sessions_map, source).unwrap();
            assert!(sessions_map.is_empty())
        }
        // # Test case #2.
        // Error: No active session
        {
            let res = funcs::delete_session(&mut sessions_map, source);
            assert!(res.is_err_and(|err| err == Error::NoActiveSession));
        }
    }

    mod utils {
        use super::{Session, SessionMap};
        use gstd::ActorId;

        pub fn sessions_map<const N: usize>(content: [(ActorId, Session); N]) -> SessionMap {
            content.into_iter().collect()
        }

        pub fn alice() -> ActorId {
            1u64.into()
        }
    }
}
