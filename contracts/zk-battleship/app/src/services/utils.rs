use core::fmt::Debug;
use gstd::{ext, format, prelude::*};

pub fn panicking<T, E: Debug, F: FnOnce() -> Result<T, E>>(f: F) -> T {
    match f() {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

pub fn panic(err: impl Debug) -> ! {
    ext::panic(&format!("{err:?}"))
}

#[macro_export]
macro_rules! declare_storage {
    (name: $name: ident, ty: $ty: ty $(,)?) => {
        $crate::declare_storage!(module: internal, name: $name, ty: $ty);
    };

    (module: $module: ident, name: $name: ident, ty: $ty: ty $(,)?) => {
        pub struct $name(());

        mod $module {
            #![allow(static_mut_refs)]
            use super::*;

            static mut INSTANCE: Option<$ty> = None;

            impl $name {
                pub fn is_set() -> bool {
                    unsafe { INSTANCE.is_some() }
                }

                pub fn set(value: $ty) -> Result<(), $ty> {
                    if Self::is_set() {
                        Err(value)
                    } else {
                        unsafe { INSTANCE = Some(value) }
                        Ok(())
                    }
                }

                pub fn default() -> Result<(), $ty> {
                    Self::set(<$ty>::new())
                }

                pub fn as_ref() -> &'static $ty {
                    unsafe {
                        INSTANCE.as_ref().unwrap_or_else(|| {
                            panic!(
                                "Storage {} should be set before accesses",
                                stringify!($name)
                            )
                        })
                    }
                }

                pub fn as_mut() -> &'static mut $ty {
                    unsafe {
                        INSTANCE.as_mut().unwrap_or_else(|| {
                            panic!(
                                "Storage {} should be set before accesses",
                                stringify!($name)
                            )
                        })
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_getter_game {
    ($field_name:ident, $player_id:expr) => {
        if let Some(game) = SingleGamesStorage::as_ref().get(&$player_id) {
            Some(game.$field_name.clone())
        } else {
            None
        }
    };
    ($player_id:expr) => {
        if let Some(game) = SingleGamesStorage::as_ref().get(&$player_id) {
            Some(game.clone())
        } else {
            None
        }
    };
}
