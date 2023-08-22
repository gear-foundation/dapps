use catalog_io::CatalogMetadata;
use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::with_meta(<CatalogMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
