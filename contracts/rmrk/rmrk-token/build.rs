use gear_wasm_builder::WasmBuilder;
use rmrk_io::RMRKMetadata;

fn main() {
    WasmBuilder::with_meta(<RMRKMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
