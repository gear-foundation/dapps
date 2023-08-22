use ft_io::FungibleTokenMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(FungibleTokenMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
