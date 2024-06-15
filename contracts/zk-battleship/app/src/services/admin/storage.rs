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
            if Self::is_set() {
                Err(())
            } else {
                unsafe {
                    INSTANCE = Some(VerificationKeyStorage {
                        vk_for_start,
                        vk_for_move,
                    })
                }
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

pub mod configuration {
    use gstd::{Decode, Encode, TypeInfo};
    pub struct ConfigurationStorage(());

    #[derive(Clone, Copy, Default, Encode, Decode, TypeInfo)]
    #[codec(crate = sails_rtl::scale_codec)]
    #[scale_info(crate = sails_rtl::scale_info)]
    pub struct Configuration {
        pub gas_for_delete_single_game: u64,
        pub gas_for_delete_multiple_game: u64,
        pub gas_for_check_time: u64,
        pub delay_for_delete_single_game: u32,
        pub delay_for_delete_multiple_game: u32,
        pub delay_for_check_time: u32,
    }

    static mut INSTANCE: Option<Configuration> = None;

    impl ConfigurationStorage {
        pub fn is_set() -> bool {
            unsafe { INSTANCE.is_some() }
        }

        pub fn set(value: Configuration) -> Result<(), Configuration> {
            if Self::is_set() {
                Err(value)
            } else {
                unsafe { INSTANCE = Some(value) }
                Ok(())
            }
        }

        pub fn default() -> Result<(), Configuration> {
            Self::set(Configuration::default())
        }

        pub fn get() -> Configuration {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe { *INSTANCE.as_ref().expect("Infallible b/c set above") }
        }

        pub fn get_mut() -> &'static mut Configuration {
            if !Self::is_set() {
                let _res = Self::default();
                debug_assert!(_res.is_ok());
            }

            unsafe { INSTANCE.as_mut().expect("Infallible b/c set above") }
        }
    }
}
