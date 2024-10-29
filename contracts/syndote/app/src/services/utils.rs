use core::fmt::Debug;
use gstd::{ext, format, msg};

pub fn panicking<T, E: Debug, F: FnOnce() -> Result<T, E>>(f: F) -> T {
    match f() {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

pub fn panic(err: impl Debug) -> ! {
    let value = msg::value();
    if value != 0 {
        msg::send(msg::source(), "", value).expect("Error during sending a value");
    }
    ext::panic(&format!("{err:?}"))
}
