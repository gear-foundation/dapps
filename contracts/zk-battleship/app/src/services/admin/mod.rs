use self::storage::{
    admin::AdminStorage, builtin_bls381::BuiltinStorage, configuration::Configuration,
    verification_key::VerificationKeyStorage,
};
use crate::services;
use crate::VerifyingKeyBytes;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{exec, msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
use sails_rtl::gstd::{
    events::{EventTrigger, GStdEventTrigger},
    gservice,
};
use storage::configuration::ConfigurationStorage;

pub mod storage;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Event {
    GameDeleted,
    GamesDeleted,
    AdminChanged,
    BuiltinAddressChanged,
    VerificationKeyChanged,
    Killed { inheritor: ActorId },
}

pub type GstdDrivenService = Service<GStdEventTrigger<Event>>;

#[derive(Clone)]
pub struct Service<X>(PhantomData<X>);

impl<X> Service<X> {
    pub fn seed(
        admin: ActorId,
        builtin_bls381: ActorId,
        verification_key_for_start: services::verify::VerifyingKeyBytes,
        verification_key_for_move: services::verify::VerifyingKeyBytes,
        config: Configuration,
    ) -> Self {
        let _res = AdminStorage::set(admin);
        debug_assert!(_res.is_ok());
        let _res = BuiltinStorage::set(builtin_bls381);
        debug_assert!(_res.is_ok());
        let _res =
            VerificationKeyStorage::set(verification_key_for_start, verification_key_for_move);
        debug_assert!(_res.is_ok());
        let _res = ConfigurationStorage::set(config);
        debug_assert!(_res.is_ok());
        Self(PhantomData)
    }
}

#[gservice]
impl<X> Service<X>
where
    X: EventTrigger<Event>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn delete_single_game(&mut self, player_address: ActorId) {
        Self::check_admin(msg::source());
        services::single::storage::SingleGamesStorage::as_mut().remove(&player_address);
        services::utils::deposit_event(Event::GameDeleted);
    }
    pub fn delete_single_games(&mut self, time: u64) {
        Self::check_admin(msg::source());
        let games = services::single::storage::SingleGamesStorage::as_mut();
        let current_time = exec::block_timestamp();
        games.retain(|_id, game| (current_time - game.start_time) <= time);
        services::utils::deposit_event(Event::GamesDeleted);
    }
    pub fn delete_multiple_game(&mut self, game_id: ActorId) {
        Self::check_admin(msg::source());
        services::multiple::storage::MultipleGamesStorage::as_mut().remove(&game_id);
        services::multiple::storage::GamePairsStorage::as_mut().retain(|_, &mut id| id != game_id);
        services::utils::deposit_event(Event::GameDeleted);
    }
    pub fn delete_multiple_games_by_time(&mut self, time: u64) {
        Self::check_admin(msg::source());
        let games = services::multiple::storage::MultipleGamesStorage::as_mut();
        let current_time = exec::block_timestamp();
        let mut ids_to_remove = Vec::new();

        games.retain(|id, game| match game.start_time {
            Some(start_time) => {
                if (current_time - start_time) > time {
                    ids_to_remove.push(*id);
                    false
                } else {
                    true
                }
            }
            None => true,
        });

        let game_pairs = services::multiple::storage::GamePairsStorage::as_mut();
        for id in ids_to_remove {
            game_pairs.retain(|_, &mut game_id| game_id != id);
        }
        services::utils::deposit_event(Event::GamesDeleted);
    }

    pub fn delete_multiple_games_in_batches(&mut self, divider: u64) {
        Self::check_admin(msg::source());
        let games = services::multiple::storage::MultipleGamesStorage::as_mut();
        let mut count = 0;
        let mut ids_to_remove = Vec::new();

        games.retain(|id, _game| {
            count += 1;
            if count % divider == 0 {
                ids_to_remove.push(*id);
                false
            } else {
                true
            }
        });

        let game_pairs = services::multiple::storage::GamePairsStorage::as_mut();
        for id in ids_to_remove {
            game_pairs.retain(|_, &mut game_id| game_id != id);
        }

        services::utils::deposit_event(Event::GamesDeleted);
    }
    pub fn change_admin(&mut self, new_admin: ActorId) {
        Self::check_admin(msg::source());
        let admin = AdminStorage::get_mut();
        *admin = new_admin;
        services::utils::deposit_event(Event::AdminChanged);
    }
    pub fn change_builtin_address(&mut self, new_builtin_address: ActorId) {
        Self::check_admin(msg::source());
        let builtin = BuiltinStorage::get_mut();
        *builtin = new_builtin_address;
        services::utils::deposit_event(Event::BuiltinAddressChanged);
    }
    pub fn change_verification_key(
        &mut self,
        new_vk_for_start: Option<services::verify::VerifyingKeyBytes>,
        new_vk_for_move: Option<services::verify::VerifyingKeyBytes>,
    ) {
        Self::check_admin(msg::source());
        if let Some(new_vk) = new_vk_for_start {
            let vk_for_start = VerificationKeyStorage::get_mut_vk_for_start();
            *vk_for_start = new_vk;
        }
        if let Some(new_vk) = new_vk_for_move {
            let vk_for_move = VerificationKeyStorage::get_mut_vk_for_move();
            *vk_for_move = new_vk;
        }
        services::utils::deposit_event(Event::VerificationKeyChanged);
    }

    pub fn kill(&mut self, inheritor: ActorId) {
        Self::check_admin(msg::source());
        services::utils::deposit_event(Event::Killed { inheritor });
        exec::exit(inheritor);
    }
    fn check_admin(source: ActorId) {
        assert!(
            source == AdminStorage::get(),
            "No permission to call this function"
        );
    }

    pub fn admin(&self) -> ActorId {
        AdminStorage::get()
    }
    pub fn builtin(&self) -> ActorId {
        BuiltinStorage::get()
    }
    pub fn verification_key(&self) -> (VerifyingKeyBytes, VerifyingKeyBytes) {
        (
            VerificationKeyStorage::get_vk_for_start().clone(),
            VerificationKeyStorage::get_vk_for_move().clone(),
        )
    }
}
