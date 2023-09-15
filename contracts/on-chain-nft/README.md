[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=on-chain-nft/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-build.yml?logo=rust&label=docs)](https://dapps.gear.rs/on_chain_nft_io)

# [On-chain NFT](https://wiki.gear-tech.io/docs/examples/onchain-nft)

### 🏗️ Building

```sh
cargo b -p "on-chain-nft*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "on-chain-nft*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "on-chain-nft*"
```
