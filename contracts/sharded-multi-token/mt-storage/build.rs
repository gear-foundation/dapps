use gear_wasm_builder::WasmBuilder;
use mt_storage_io::MTStorageMetadata;

fn main() {
    WasmBuilder::with_meta(<MTStorageMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
