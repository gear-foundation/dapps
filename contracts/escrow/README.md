[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=escrow/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/escrow_io)

# [Escrow](https://wiki.gear-tech.io/docs/examples/DeFi/escrow)

### 🏗️ Building

```sh
cargo b -p "escrow*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "escrow*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "escrow*"
```
