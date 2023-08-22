use escrow_io::EscrowMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(EscrowMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
