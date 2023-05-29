use gear_wasm_builder::WasmBuilder;
use nft_master_io::NFTMasterMetadata;

fn main() {
    WasmBuilder::with_meta(<NFTMasterMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
