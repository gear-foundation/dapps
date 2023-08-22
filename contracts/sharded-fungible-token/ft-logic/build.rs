use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::with_meta(<ft_logic_io::FLogicMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
