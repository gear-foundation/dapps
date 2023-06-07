use channel_io::ChannelMetadata;
use gear_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::with_meta(<ChannelMetadata as gmeta::Metadata>::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}
