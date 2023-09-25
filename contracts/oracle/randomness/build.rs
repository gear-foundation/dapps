use oracle_randomness_io::RandomnessOracleMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<RandomnessOracleMetadata>();
}
