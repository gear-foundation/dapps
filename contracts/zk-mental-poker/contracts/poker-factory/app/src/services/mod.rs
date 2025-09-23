use gstd::prog::ProgramGenerator;
use poker_client::{SessionConfig, SignatureInfo, ZkPublicKey};
use sails_rs::collections::{HashMap, HashSet};
use sails_rs::gstd::msg;
use sails_rs::prelude::*;
mod utils;
use crate::services::utils::panic;
use pts_client::pts::io as pts_io;
use sails_rs::calls::ActionIo;

#[derive(Debug, Clone)]
struct Storage {
    lobbies: HashMap<ActorId, LobbyConfig>,
    admins: HashSet<ActorId>,
    config: Config,
    pts_actor_id: ActorId,
    zk_verification_id: ActorId,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Config {
    pub lobby_code_id: CodeId,
    pub gas_for_program: u64,
    pub gas_for_reply_deposit: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct LobbyConfig {
    admin_id: ActorId,
    admin_name: String,
    lobby_name: String,
    small_blind: u128,
    big_blind: u128,
    starting_bank: u128,
    time_per_move_ms: u64,
}

static mut STORAGE: Option<Storage> = None;

#[event]
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    LobbyCreated {
        lobby_address: ActorId,
        admin: ActorId,
        pk: ZkPublicKey,
        lobby_config: LobbyConfig,
    },
    LobbyDeleted {
        lobby_address: ActorId,
    },
    ConfigChanged {
        config: Config,
    },
    ZkVerificationIdChanged {
        zk_verification_id: ActorId,
    },
    PtsActorIdChanged {
        pts_actor_id: ActorId,
    },
}

pub struct PokerFactoryService(());

#[allow(clippy::new_without_default)]
impl PokerFactoryService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn init(config: Config, pts_actor_id: ActorId, zk_verification_id: ActorId) -> Self {
        unsafe {
            STORAGE = Some(Storage {
                admins: HashSet::from([msg::source()]),
                config,
                lobbies: HashMap::new(),
                pts_actor_id,
                zk_verification_id,
            });
        }
        Self(())
    }
    fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[sails_rs::service(events = Event)]
impl PokerFactoryService {
    /// Creates new poker lobby with provided config.
    ///
    /// Panics if:
    /// - Insufficient PTS balance
    /// - Program creation fails
    ///
    /// Performs:
    /// 1. Checks player's PTS balance
    /// 2. Deploys new lobby program
    /// 3. Sets lobby as PTS admin
    /// 4. Transfers starting bank to lobby
    /// 5. Stores lobby info and emits LobbyCreated event
    #[export]
    pub async fn create_lobby(
        &mut self,
        init_lobby: LobbyConfig,
        pk: ZkPublicKey,
        session: Option<SignatureInfo>,
    ) {
        let storage = self.get_mut();
        let msg_src = msg::source();

        if msg::value() != 1_000_000_000_000 {
            panic!("Wrong value to create a lobby");
        }

        if init_lobby.time_per_move_ms < 15_000 {
            panic!("Timer less than 15s");
        }

        let request = pts_io::GetBalance::encode_call(msg_src);

        let bytes_reply_balance = msg::send_bytes_for_reply(storage.pts_actor_id, request, 0, 0)
            .expect("Error in async message to PTS contract")
            .await
            .expect("PTS: Error getting balance from the contract");

        let balance: u128 = pts_io::GetBalance::decode_reply(bytes_reply_balance).unwrap();

        if balance < init_lobby.starting_bank {
            panic!("Low pts balance");
        }

        let session_config = SessionConfig {
            gas_to_delete_session: 10_000_000_000,
            minimum_session_duration_ms: 180_000,
            ms_per_block: 3_000,
        };
        let payload = [
            "New".encode(),
            init_lobby.encode(),
            session_config.encode(),
            storage.pts_actor_id.encode(),
            pk.encode(),
            session.encode(),
            storage.zk_verification_id.encode(),
        ]
        .concat();
        let create_program_future = ProgramGenerator::create_program_bytes_with_gas_for_reply(
            storage.config.lobby_code_id,
            payload,
            storage.config.gas_for_program,
            0,
            storage.config.gas_for_reply_deposit,
        )
        .unwrap_or_else(|e| panic(e));

        let (lobby_address, _) = create_program_future.await.unwrap_or_else(|e| panic(e));

        let request = pts_io::AddAdmin::encode_call(lobby_address);

        msg::send_bytes_for_reply(storage.pts_actor_id, request, 0, 0)
            .expect("Error in async message to PTS contract")
            .await
            .expect("PTS: Error adding new admin");

        let request =
            pts_io::Transfer::encode_call(msg_src, lobby_address, init_lobby.starting_bank);

        msg::send_bytes_for_reply(storage.pts_actor_id, request, 0, 0)
            .expect("Error in async message to PTS contract")
            .await
            .expect("PTS: Error transfer points to contract");

        storage.lobbies.insert(lobby_address, init_lobby.clone());

        self.emit_event(Event::LobbyCreated {
            lobby_address,
            admin: msg_src,
            pk,
            lobby_config: init_lobby,
        })
        .expect("Notification Error");
    }

    /// Deletes lobby from registry. Admin or lobby itself only.
    /// Panics if:
    /// - Lobby doesn't exist
    /// - Caller lacks permissions
    ///
    /// Emits LobbyDeleted event on success.
    #[export]
    pub async fn delete_lobby(&mut self, lobby_address: ActorId) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        let lobby = storage
            .lobbies
            .get(&lobby_address)
            .expect("Lobby must be exist");
        if msg_src != lobby.admin_id
            && msg_src != lobby_address
            && !storage.admins.contains(&msg_src)
        {
            panic!("Access denied");
        }
        storage.lobbies.remove(&lobby_address);

        self.emit_event(Event::LobbyDeleted { lobby_address })
            .expect("Notification Error");
    }

    #[export]
    pub async fn change_config(&mut self, config: Config) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if !storage.admins.contains(&msg_src) {
            panic!("Access denied");
        }
        storage.config = config.clone();

        self.emit_event(Event::ConfigChanged { config })
            .expect("Notification Error");
    }

    #[export]
    pub async fn change_zk_verification_id(&mut self, zk_verification_id: ActorId) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if !storage.admins.contains(&msg_src) {
            panic!("Access denied");
        }
        storage.zk_verification_id = zk_verification_id;

        self.emit_event(Event::ZkVerificationIdChanged { zk_verification_id })
            .expect("Notification Error");
    }

    #[export]
    pub async fn change_pts_actor_id(&mut self, pts_actor_id: ActorId) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if !storage.admins.contains(&msg_src) {
            panic!("Access denied");
        }
        storage.pts_actor_id = pts_actor_id;

        self.emit_event(Event::PtsActorIdChanged { pts_actor_id })
            .expect("Notification Error");
    }

    #[export]
    pub fn add_admin(&mut self, new_admin_id: ActorId) {
        let storage = self.get_mut();
        storage.admins.insert(new_admin_id);
    }

    #[export]
    pub fn delete_admin(&mut self, id: ActorId) {
        let storage = self.get_mut();
        storage.admins.remove(&id);
    }

    #[export]
    pub fn pts_actor_id(&self) -> ActorId {
        self.get().pts_actor_id
    }

    #[export]
    pub fn lobbies(&self) -> Vec<(ActorId, LobbyConfig)> {
        self.get().lobbies.clone().into_iter().collect()
    }

    #[export]
    pub fn admins(&self) -> Vec<ActorId> {
        self.get().admins.clone().into_iter().collect()
    }

    #[export]
    pub fn config(&self) -> Config {
        self.get().config.clone()
    }
}
