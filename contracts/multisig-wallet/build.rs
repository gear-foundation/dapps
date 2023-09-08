use multisig_wallet_io::ContractMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<ContractMetadata>();
}
