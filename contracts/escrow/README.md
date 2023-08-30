[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=escrow/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/escrow_io)

# [Escrow](https://wiki.gear-tech.io/docs/examples/escrow)

### 🏗️ Building

```sh
cargo b -p "escrow*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "escrow*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "escrow*" -- --include-ignored
```
