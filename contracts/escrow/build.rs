use escrow_io::EscrowMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<EscrowMetadata>();
}
