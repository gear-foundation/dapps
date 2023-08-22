use gear_wasm_builder::WasmBuilder;
use mt_logic_io::MTLogicMetadata;

fn main() {
    WasmBuilder::with_meta(<MTLogicMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
