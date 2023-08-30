[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=dex/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/dex_io)

# [DEX (Decentralized Exchange)](https://wiki.gear-tech.io/docs/examples/dex)

### 🏗️ Building

```sh
cargo b -p "dex*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "dex*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "dex*" -- --include-ignored
```
