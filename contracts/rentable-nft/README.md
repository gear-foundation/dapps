[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=rentable-nft/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-build.yml?logo=rust&label=docs)](https://dapps.gear.rs/rentable_nft_io)

# [Rentable NFT](https://wiki.gear-tech.io/docs/examples/gnft-4907)

### 🏗️ Building

```sh
cargo b -p "rentable-nft*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "rentable-nft*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "rentable-nft*"
```
