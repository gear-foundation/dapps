use gear_wasm_builder::WasmBuilder;
use staking_io::StakingMetadata;

fn main() {
    WasmBuilder::with_meta(<StakingMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
