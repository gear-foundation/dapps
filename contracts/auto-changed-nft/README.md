[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=auto-changed-nft/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/auto_changed_nft_io)

# [Auto-changed NFT](https://wiki.gear-tech.io/docs/examples/NFTs/dynamic-nft#examples)

An example of Auto-Changed NFT (modified [Dynamic NFT](../dynamic-nft)).

### 🏗️ Building

```sh
cargo b -r -p "auto-changed-nft*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -r -p "auto-changed-nft*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -r -p "auto-changed-nft*"
```
