use anyhow::{anyhow, Result};
use std::env;
use xshell::Shell;

fn main() -> Result<()> {
    let Some(command) = env::args().nth(1) else {
        return Err(anyhow!("command wasn't given"));
    };

    let sh = Shell::new()?;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let node = || -> Result<_> {
        if xshell::cmd!(sh, "[ -e {manifest_dir}'/../target/tmp/gear' ]")
            .quiet()
            .run()
            .is_err()
        {
            xshell::cmd!(
                sh,
                "bash -c 'curl -L https://get.gear.rs/gear-v0.3.3-x86_64-unknown-linux-gnu.tar.xz -o - | tar xJ -C '{manifest_dir}'/../target/tmp'"
            )
            .run()?;
        }

        Ok(())
    };

    let docs = || -> Result<_> {
        xshell::cmd!(
            sh,
            "cargo d --no-deps -p '*-io' -p '*-state' -p rmrk-types -p 'gear-lib*'"
        )
        .env("__GEAR_WASM_BUILDER_NO_BUILD", "")
        .run()?;

        Ok(())
    };

    match command.as_str() {
        "node" => node()?,
        "ci" => {
            xshell::cmd!(sh, "cargo fmt --all --check").run()?;
            docs()?;
            xshell::cmd!(sh, "cargo clippy --all-targets -- -Dwarnings").run()?;
            node()?;
            xshell::cmd!(sh, "cargo t").run()?;
        }
        "docs" => docs()?,
        _ => return Err(anyhow!("unknown command")),
    }

    Ok(())
}
