use sharded_fungible_token_io::FMainTokenMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<FMainTokenMetadata>();
}
