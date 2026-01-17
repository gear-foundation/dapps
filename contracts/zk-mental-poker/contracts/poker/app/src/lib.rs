#![no_std]

pub mod services;

use sails_rs::cell::RefCell;
use sails_rs::prelude::*;

use services::game::{Config as LobbyConfig, PokerService, ZkPublicKey};
use services::session::{SessionConfig, SessionService, SessionStorage, SignatureInfo};

pub struct PokerProgram {
    session_storage: RefCell<SessionStorage>,
}

#[sails_rs::program]
impl PokerProgram {
    pub async fn new(
        config: LobbyConfig,
        session_config: SessionConfig,
        pts_actor_id: ActorId,
        pk: ZkPublicKey,
        session_for_admin: Option<SignatureInfo>,
        zk_verification_id: ActorId,
    ) -> Self {
        let admin_id = config.admin_id;

        PokerService::init(config, pts_actor_id, pk, zk_verification_id);

        let program = Self {
            session_storage: RefCell::new(SessionStorage::new(session_config)),
        };

        if let Some(SignatureInfo {
            signature_data,
            signature,
        }) = session_for_admin
        {
            let mut session = SessionService::new(&program.session_storage);
            session
                .create_session_for_admin(signature_data, signature, admin_id)
                .expect("Failed to create admin session");
        }

        program
    }

    pub fn poker(&self) -> PokerService<'_> {
        PokerService::new(&self.session_storage)
    }

    pub fn session(&self) -> SessionService<'_> {
        SessionService::new(&self.session_storage)
    }
}
