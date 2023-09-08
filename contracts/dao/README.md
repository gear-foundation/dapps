[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=dao/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/dao_io)

# [DAO](https://wiki.gear-tech.io/docs/examples/DAO)

### 🏗️ Building

```sh
cargo b -p dao -p "dao-[!l]*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p dao -p "dao-[!l]*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p dao -p "dao-[!l]*" -- --include-ignored
```
