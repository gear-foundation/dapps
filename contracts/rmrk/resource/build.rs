use gear_wasm_builder::WasmBuilder;
use resource_io::ResourceMetadata;

fn main() {
    WasmBuilder::with_meta(<ResourceMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
