use gear_wasm_builder::WasmBuilder;
use mt_main_io::MTMainMetadata;

fn main() {
    WasmBuilder::with_meta(<MTMainMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
