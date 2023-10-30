[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=dex/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/dex_io)

# [DEX (Decentralized Exchange)](https://wiki.gear-tech.io/docs/examples/DeFi/dex)

### 🏗️ Building

```sh
cargo b -p "dex*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "dex*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "dex*"
```
