use staking_io::StakingMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<StakingMetadata>();
}
