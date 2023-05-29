use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use launch_io::ProgramMetadata;

fn main() {
    WasmBuilder::with_meta(ProgramMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
