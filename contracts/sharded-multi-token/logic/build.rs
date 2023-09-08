use sharded_multi_token_logic_io::MTLogicMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<MTLogicMetadata>();
}
