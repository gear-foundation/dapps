use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use varatube_io::SubscriptionMetadata;

fn main() {
    WasmBuilder::with_meta(SubscriptionMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
