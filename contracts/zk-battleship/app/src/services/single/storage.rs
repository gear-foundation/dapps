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
