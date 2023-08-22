use gear_wasm_builder::WasmBuilder;
use market_io::MarketMetadata;

fn main() {
    WasmBuilder::with_meta(<MarketMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
