use dynamic_nft_io::NFTMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(NFTMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
