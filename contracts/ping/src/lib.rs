#![no_std]

use gstd::{debug, msg, prelude::*};

static mut MESSAGE_LOG: Vec<String> = vec![];

#[no_mangle]
extern "C" fn handle() {
    let new_msg = String::from_utf8(msg::load_bytes().expect("Invalid message"))
        .expect("Unable to create string");

    if new_msg == "PING" {
        msg::reply_bytes("PONG", 0).expect("Unable to reply");
    }

    unsafe {
        MESSAGE_LOG.push(new_msg);

        debug!("{:?} total message(s) stored: ", MESSAGE_LOG.len());

        for log in &MESSAGE_LOG {
            debug!(log);
        }
    }
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

        let res = program.send_bytes(42, "INIT");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "PING");
        let log = Log::builder().source(1).dest(42).payload_bytes("PONG");
        assert!(res.contains(&log));
    }
}
