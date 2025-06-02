use sails_rs::{fmt::Debug, format};

pub fn panicking<T, E: Debug>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

pub fn panic(err: impl Debug) -> ! {
    panic!("{}", format!("{err:?}"))
}
