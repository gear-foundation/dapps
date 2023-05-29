use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new_metawasm()
        .exclude_features(["binary-vendor"])
        .build()
}
