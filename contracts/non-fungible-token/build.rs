use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use nft_io::NFTMetadata;

fn main() {
    WasmBuilder::with_meta(NFTMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
