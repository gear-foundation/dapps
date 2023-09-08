use dutch_auction_io::io::AuctionMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<AuctionMetadata>();
}
