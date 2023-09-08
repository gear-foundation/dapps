use sharded_fungible_token_storage_io::FTStorageMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<FTStorageMetadata>();
}
