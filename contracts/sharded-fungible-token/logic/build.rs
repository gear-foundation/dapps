use sharded_fungible_token_logic_io::FLogicMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<FLogicMetadata>();
}
