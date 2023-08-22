use auction_io::io::AuctionMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(AuctionMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
