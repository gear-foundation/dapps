[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=nft-marketplace/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/nft_marketplace_io)

# [NFT marketplace](https://wiki.gear-tech.io/docs/examples/nft-marketplace/marketplace)

### 🏗️ Building

```sh
cargo b -p "nft-marketplace*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "nft-marketplace*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "nft-marketplace*" -- --include-ignored
```
