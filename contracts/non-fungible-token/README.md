[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=non-fungible-token/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/non_fungible_token_io)

# [Non-fungible token](https://wiki.gear-tech.io/docs/examples/gnft-721)

### 🏗️ Building

```sh
cargo b -p "non-fungible-token*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "non-fungible-token*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "non-fungible-token*" -- --include-ignored
```
