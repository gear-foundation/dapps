use self::storage::{
    admin::AdminStorage, builtin_bls381::BuiltinStorage, verification_key::VerificationKeyStorage,
};
use crate::services;
use crate::VerifyingKeyBytes;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{exec, msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
use sails_rtl::gstd::{
    events::{EventTrigger, GStdEventTrigger},
    gservice,
};

pub mod storage;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Event {
    GameDeleted,
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
    ) -> Self {
        let _res = AdminStorage::set(admin);
        debug_assert!(_res.is_ok());
        let _res = BuiltinStorage::set(builtin_bls381);
        debug_assert!(_res.is_ok());
        let _res =
            VerificationKeyStorage::set(verification_key_for_start, verification_key_for_move);
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

    pub fn delete_game(&mut self, player_address: ActorId) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the admin can change the program state"
        );
        services::single::storage::SingleGamesStorage::as_mut().remove(&player_address);
        services::utils::deposit_event(Event::GameDeleted);
    }
    pub fn change_admin(&mut self, new_admin: ActorId) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );
        let admin = AdminStorage::get_mut();
        *admin = new_admin;
        services::utils::deposit_event(Event::AdminChanged);
    }
    pub fn change_builtin_address(&mut self, new_builtin_address: ActorId) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );
        let builtin = BuiltinStorage::get_mut();
        *builtin = new_builtin_address;
        services::utils::deposit_event(Event::BuiltinAddressChanged);
    }
    pub fn change_verification_key(
        &mut self,
        new_vk_for_start: Option<services::verify::VerifyingKeyBytes>,
        new_vk_for_move: Option<services::verify::VerifyingKeyBytes>,
    ) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );
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
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );

        services::utils::deposit_event(Event::Killed { inheritor });
        exec::exit(inheritor);
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
