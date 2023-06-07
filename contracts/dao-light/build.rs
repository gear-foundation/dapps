use dao_light_io::*;
use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::with_meta(<DaoLightMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
