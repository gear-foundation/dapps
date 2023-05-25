use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use multitoken_io::MultitokenMetadata;

fn main() {
    WasmBuilder::with_meta(MultitokenMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
