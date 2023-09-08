use dao_io::DaoMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<DaoMetadata>();
}
