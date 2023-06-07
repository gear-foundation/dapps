use gear_wasm_builder::WasmBuilder;
use nft_pixelboard_io::ContractMetadata;

fn main() {
    WasmBuilder::with_meta(<ContractMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
