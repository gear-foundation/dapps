use crowdsale_io::CrowdsaleMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(CrowdsaleMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
