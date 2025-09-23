use core::fmt::Debug;
use gstd::{ext, format};

pub fn panic(err: impl Debug) -> ! {
    ext::panic_bytes(format!("{err:?}").as_bytes())
}
