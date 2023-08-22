use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use onchain_nft_io::ContractMetadata;

fn main() {
    WasmBuilder::with_meta(ContractMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
