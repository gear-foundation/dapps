[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=concert/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-build.yml?logo=rust&label=docs)](https://dapps.gear.rs/concert_io)

# [Concert](https://wiki.gear-tech.io/docs/examples/concert)

### 🏗️ Building

```sh
cargo b -p "concert*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "concert*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "concert*"
```
