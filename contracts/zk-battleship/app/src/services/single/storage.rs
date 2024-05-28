use super::{SessionMap, SingleGamesMap};
crate::declare_storage!(module: single_games, name: SingleGamesStorage, ty: SingleGamesMap);

crate::declare_storage!(module: sessions, name: SessionsStorage, ty: SessionMap);

pub mod admin {
    use gstd::ActorId;
    pub struct AdminStorage(());

    static mut INSTANCE: Option<ActorId> = None;

    impl AdminStorage {
        pub fn is_set() -> bool {
            unsafe { INSTANCE.is_some() }
        }

        pub fn set(value: ActorId) -> Result<(), ActorId> {
            if Self::is_set() {
                Err(value)
            } else {
                unsafe { INSTANCE = Some(value) }
                Ok(())
            }
        }

        pub fn default() -> Result<(), ActorId> {
            Self::set(ActorId::zero())
        }

        pub fn get() -> ActorId {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe { *INSTANCE.as_ref().expect("Infallible b/c set above") }
        }

        pub fn get_mut() -> &'static mut ActorId {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe { INSTANCE.as_mut().expect("Infallible b/c set above") }
        }
    }
}

pub mod builtin_bls381 {
    use gstd::ActorId;
    pub struct BuiltinStorage(());

    static mut INSTANCE: Option<ActorId> = None;

    impl BuiltinStorage {
        pub fn is_set() -> bool {
            unsafe { INSTANCE.is_some() }
        }

        pub fn set(value: ActorId) -> Result<(), ActorId> {
            if Self::is_set() {
                Err(value)
            } else {
                unsafe { INSTANCE = Some(value) }
                Ok(())
            }
        }

        pub fn default() -> Result<(), ActorId> {
            Self::set(ActorId::zero())
        }

        pub fn get() -> ActorId {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe { *INSTANCE.as_ref().expect("Infallible b/c set above") }
        }

        pub fn get_mut() -> &'static mut ActorId {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe { INSTANCE.as_mut().expect("Infallible b/c set above") }
        }
    }
}

pub mod verification_key {
    use crate::services::verify::VerifyingKeyBytes;
    use gstd::Vec;

    pub struct VerificationKeyStorage {
        pub vk_for_start: VerifyingKeyBytes,
        pub vk_for_move: VerifyingKeyBytes,
    }

    static mut INSTANCE: Option<VerificationKeyStorage> = None;

    impl VerificationKeyStorage {
        pub fn is_set() -> bool {
            unsafe { INSTANCE.is_some() }
        }

        pub fn set(
            vk_for_start: VerifyingKeyBytes,
            vk_for_move: VerifyingKeyBytes,
        ) -> Result<(), ()> {
            let storage = VerificationKeyStorage {
                vk_for_start,
                vk_for_move,
            };
            if Self::is_set() {
                Err(())
            } else {
                unsafe { INSTANCE = Some(storage) }
                Ok(())
            }
        }

        pub fn default() -> Result<(), ()> {
            let default_vk = VerifyingKeyBytes {
                alpha_g1_beta_g2: Vec::new(),
                gamma_g2_neg_pc: Vec::new(),
                delta_g2_neg_pc: Vec::new(),
                ic: Vec::with_capacity(4),
            };
            Self::set(default_vk.clone(), default_vk)
        }

        pub fn get_vk_for_start() -> &'static VerifyingKeyBytes {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe {
                &INSTANCE
                    .as_ref()
                    .expect("Infallible b/c set above")
                    .vk_for_start
            }
        }

        pub fn get_mut_vk_for_start() -> &'static mut VerifyingKeyBytes {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe {
                &mut INSTANCE
                    .as_mut()
                    .expect("Infallible b/c set above")
                    .vk_for_start
            }
        }

        pub fn get_vk_for_move() -> &'static VerifyingKeyBytes {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe {
                &INSTANCE
                    .as_ref()
                    .expect("Infallible b/c set above")
                    .vk_for_move
            }
        }

        pub fn get_mut_vk_for_move() -> &'static mut VerifyingKeyBytes {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe {
                &mut INSTANCE
                    .as_mut()
                    .expect("Infallible b/c set above")
                    .vk_for_move
            }
        }
    }
}
