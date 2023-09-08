[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=dynamic-nft/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/dynamic_nft_io)

# [Dynamic NFT](https://wiki.gear-tech.io/docs/examples/dynamic-nft)

### 🏗️ Building

```sh
cargo b -p "dynamic-nft*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "dynamic-nft*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "dynamic-nft*" -- --include-ignored
```
