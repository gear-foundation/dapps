use sharded_multi_token_storage_io::MTStorageMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<MTStorageMetadata>();
}
