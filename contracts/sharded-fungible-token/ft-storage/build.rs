use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::with_meta(<ft_storage_io::FTStorageMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
