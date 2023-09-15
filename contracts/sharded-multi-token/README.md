[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=sharded-multi-token/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/sharded_multi_token_io)

# [Sharded multi token](https://wiki.gear-tech.io/docs/examples/gmt-1155)

An advanced version of multi token that supports sharding.

### 🏗️ Building

```sh
cargo b -p "sharded-multi-token*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "sharded-multi-token*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "sharded-multi-token*"
```
