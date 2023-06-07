use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use tequila_io::ContractMetadata;

fn main() {
    WasmBuilder::with_meta(ContractMetadata::repr()).build();
}
