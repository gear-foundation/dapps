#![allow(static_mut_refs)]
use gstd::{exec, msg};
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::service,
    prelude::*,
};
mod funcs;
use crate::services;
use vnft_service::{utils::TokenId, Service as VnftService, Storage};

#[derive(Default)]
pub struct ExtendedStorage {
    token_id: TokenId,
    minters: HashSet<ActorId>,
    burners: HashSet<ActorId>,
    admins: HashSet<ActorId>,
    token_metadata_by_id: HashMap<TokenId, TokenMetadata>,
    gas_for_one_time_updating: u64,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct TokenMetadata {
    pub name: String,
    pub description: String,
    pub current_media_index: u64,
    pub media: Vec<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub reference: String,  // URL to an off-chain JSON file with more info
}

static mut EXTENDED_STORAGE: Option<ExtendedStorage> = None;

#[event]
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    Minted {
        to: ActorId,
        token_metadata: TokenMetadata,
    },
    Burned {
        from: ActorId,
        token_id: TokenId,
    },
    MetadataStartedUpdaing {
        updates_count: u32,
        update_period_in_blocks: u32,
        token_id: TokenId,
    },
    MetadataUpdated {
        token_id: TokenId,
        current_media_index: u64,
    },
}
#[derive(Clone)]
pub struct ExtendedService {
    dynamic_nft: VnftService,
}

impl ExtendedService {
    pub fn new() -> Self {
        Self {
            dynamic_nft: VnftService::new(),
        }
    }
    pub fn init(name: String, symbol: String, gas_for_one_time_updating: u64) -> Self {
        let admin = msg::source();
        unsafe {
            EXTENDED_STORAGE = Some(ExtendedStorage {
                admins: [admin].into(),
                minters: [admin].into(),
                burners: [admin].into(),
                gas_for_one_time_updating,
                ..Default::default()
            });
        };
        ExtendedService {
            dynamic_nft: <VnftService>::init(name, symbol),
        }
    }

    pub fn get_mut(&mut self) -> &'static mut ExtendedStorage {
        unsafe {
            EXTENDED_STORAGE
                .as_mut()
                .expect("Extended vft is not initialized")
        }
    }
    pub fn get(&self) -> &'static ExtendedStorage {
        unsafe {
            EXTENDED_STORAGE
                .as_ref()
                .expect("Extended vft is not initialized")
        }
    }
}

impl From<ExtendedService> for VnftService {
    fn from(value: ExtendedService) -> Self {
        value.dynamic_nft
    }
}

#[service(extends = VnftService, events = Event)]
impl ExtendedService {
    #[export]
    pub fn mint(&mut self, to: ActorId, token_metadata: TokenMetadata) {
        if !self.get().minters.contains(&msg::source()) {
            panic!("Not allowed to mint")
        };
        if token_metadata.media.len() < (token_metadata.current_media_index + 1) as usize {
            panic!("Wrong value of current media index")
        }
        services::utils::panicking(|| {
            funcs::mint(
                Storage::owner_by_id(),
                Storage::tokens_for_owner(),
                &mut self.get_mut().token_metadata_by_id,
                &mut self.get_mut().token_id,
                to,
                token_metadata.clone(),
            )
        });
        self.emit_event(Event::Minted { to, token_metadata })
            .expect("Notification Error");
    }

    #[export]
    pub fn burn(&mut self, from: ActorId, token_id: TokenId) {
        if !self.get().burners.contains(&msg::source()) {
            panic!("Not allowed to burn")
        };
        services::utils::panicking(|| {
            funcs::burn(
                Storage::owner_by_id(),
                Storage::tokens_for_owner(),
                Storage::token_approvals(),
                &mut self.get_mut().token_metadata_by_id,
                token_id,
            )
        });
        self.emit_event(Event::Burned { from, token_id })
            .expect("Notification Error");
    }

    #[export]
    pub fn grant_admin_role(&mut self, to: ActorId) {
        self.ensure_is_admin();
        self.get_mut().admins.insert(to);
    }

    #[export]
    pub fn grant_minter_role(&mut self, to: ActorId) {
        self.ensure_is_admin();
        self.get_mut().minters.insert(to);
    }

    #[export]
    pub fn grant_burner_role(&mut self, to: ActorId) {
        self.ensure_is_admin();
        self.get_mut().burners.insert(to);
    }

    #[export]
    pub fn revoke_admin_role(&mut self, from: ActorId) {
        self.ensure_is_admin();
        self.get_mut().admins.remove(&from);
    }

    #[export]
    pub fn revoke_minter_role(&mut self, from: ActorId) {
        self.ensure_is_admin();
        self.get_mut().minters.remove(&from);
    }

    #[export]
    pub fn revoke_burner_role(&mut self, from: ActorId) {
        self.ensure_is_admin();
        self.get_mut().burners.remove(&from);
    }

    #[export]
    pub fn start_metadata_update(
        &mut self,
        updates_count: u32,
        update_period_in_blocks: u32,
        token_id: TokenId,
    ) {
        let msg_src = msg::source();
        if updates_count == 0 {
            panic!("Updates count cannot be zero")
        }
        if update_period_in_blocks == 0 {
            panic!("Updates period cannot be zero")
        }
        services::utils::panicking(|| {
            funcs::start_metadata_update(
                self.get().gas_for_one_time_updating,
                Storage::owner_by_id(),
                &mut self.get_mut().token_metadata_by_id,
                token_id,
                msg_src,
                updates_count,
                update_period_in_blocks,
            )
        });
        self.emit_event(Event::MetadataStartedUpdaing {
            updates_count,
            update_period_in_blocks,
            token_id,
        })
        .expect("Notification Error");
    }

    #[export]
    pub fn update_metadata(
        &mut self,
        token_id: TokenId,
        owner: ActorId,
        update_period: u32,
        updates_count: u32,
    ) {
        if msg::source() != exec::program_id() {
            panic!("This message can only be sent by the programme")
        }

        let current_media_index = services::utils::panicking(|| {
            funcs::update_metadata(
                Storage::owner_by_id(),
                &mut self.get_mut().token_metadata_by_id,
                token_id,
                owner,
                update_period,
                updates_count,
            )
        });
        self.emit_event(Event::MetadataUpdated {
            token_id,
            current_media_index,
        })
        .expect("Notification Error");
    }

    #[export]
    pub fn minters(&self) -> Vec<ActorId> {
        self.get().minters.clone().into_iter().collect()
    }

    #[export]
    pub fn burners(&self) -> Vec<ActorId> {
        self.get().burners.clone().into_iter().collect()
    }

    #[export]
    pub fn admins(&self) -> Vec<ActorId> {
        self.get().admins.clone().into_iter().collect()
    }

    #[export]
    pub fn token_id(&self) -> TokenId {
        self.get().token_id
    }

    #[export]
    pub fn token_metadata_by_id(&self, token_id: TokenId) -> Option<TokenMetadata> {
        self.get().token_metadata_by_id.get(&token_id).cloned()
    }

    #[export]
    pub fn tokens_for_owner(&self, owner: ActorId) -> Vec<(TokenId, TokenMetadata)> {
        Storage::tokens_for_owner()
            .get(&owner)
            .unwrap_or(&HashSet::new())
            .iter()
            .filter_map(|token_id| {
                self.token_metadata_by_id(*token_id)
                    .map(|metadata| (*token_id, metadata))
            })
            .collect()
    }
}

impl ExtendedService {
    fn ensure_is_admin(&self) {
        if !self.get().admins.contains(&msg::source()) {
            panic!("Not admin")
        };
    }
}
