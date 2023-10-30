[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=nft-marketplace/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/nft_marketplace_io)

# [NFT marketplace](https://wiki.gear-tech.io/docs/examples/NFTs/nft-marketplace/marketplace/)

### 🏗️ Building

```sh
cargo b -p "nft-marketplace*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "nft-marketplace*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "nft-marketplace*"
```
