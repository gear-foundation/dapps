use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use tic_tac_toe_io::ContractMetadata;

fn main() {
    WasmBuilder::with_meta(ContractMetadata::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
