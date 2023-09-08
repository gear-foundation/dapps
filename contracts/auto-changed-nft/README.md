[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=auto-changed-nft/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/auto_changed_nft_io)

# [Auto-changed NFT](https://wiki.gear-tech.io/docs/examples/dynamic-nft/#examples)

An example of Auto-Changed NFT (modified [Dynamic NFT](../dynamic-nft)).

### 🏗️ Building

```sh
cargo b -p "auto-changed-nft*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "auto-changed-nft*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "auto-changed-nft*" -- --include-ignored
```
