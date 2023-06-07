use base_io::BaseMetadata;
use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::with_meta(<BaseMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
