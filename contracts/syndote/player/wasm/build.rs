use player_app::PlayerProgram;
use sails_idl_gen::program;
use std::{env, fs::File, path::PathBuf};

fn main() {
    gear_wasm_builder::build();

    let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let idl_file_path = manifest_dir_path.join("syndote_player.idl");

    let idl_file = File::create(idl_file_path).unwrap();

    program::generate_idl::<PlayerProgram>(idl_file).unwrap();
}
