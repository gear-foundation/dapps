[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=multi-token/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/multi_token_io)

# Multi token

https://eips.ethereum.org/EIPS/eip-1155

### 🏗️ Building

```sh
cargo b -p "multi-token*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "multi-token*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "multi-token*" -- --include-ignored
```
