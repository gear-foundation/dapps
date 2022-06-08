#![no_std]

use gstd::{debug, msg, prelude::*};

#[no_mangle]
pub unsafe extern "C" fn handle() {
    debug!("handle()");
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid handle message");

    if payload == "Hello" {
        msg::reply(b"World", 0).unwrap();
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid init message");
    debug!("init(): {}", payload);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gtest::{Log, Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "Let's start");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        let log = Log::builder().source(1).dest(42).payload_bytes("World");
        assert!(res.contains(&log));
    }
}
