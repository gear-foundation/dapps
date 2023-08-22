fn main() {
    gear_wasm_builder::WasmBuilder::new_metawasm()
        .exclude_features(["binary-vendor"])
        .build();
}
