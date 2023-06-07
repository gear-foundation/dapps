use dex_pair_io::ContractMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(ContractMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build()
}
