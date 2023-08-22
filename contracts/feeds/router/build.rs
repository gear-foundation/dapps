use gear_wasm_builder::WasmBuilder;
use router_io::RouterMetadata;

fn main() {
    WasmBuilder::with_meta(<RouterMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
