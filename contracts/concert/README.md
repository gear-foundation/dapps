[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=concert/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/concert_io)

# [Concert](https://wiki.gear-tech.io/docs/examples/concert)

### 🏗️ Building

```sh
cargo b -p "concert*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "concert*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "concert*" -- --include-ignored
```
