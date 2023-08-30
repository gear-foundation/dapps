use anyhow::{anyhow, Result};
use std::env;
use xshell::Shell;

fn main() -> Result<()> {
    let Some(command) = env::args().nth(1) else {
        return Err(anyhow!("command wasn't given"));
    };

    let shell = Shell::new()?;
    let man = env!("CARGO_MANIFEST_DIR");

    match command.as_str() {
        "node" => xshell::cmd!(
            shell,
            "bash -c 'curl -L https://get.gear.rs/vara-testnet-x86_64-unknown-linux-gnu.tar.xz -o - | tar xJ -C '{man}'/../target/tmp'"
        ).quiet()
        .run()
        .map_err(Into::into),
        _ => Err(anyhow!("unknown command")),
    }
}
