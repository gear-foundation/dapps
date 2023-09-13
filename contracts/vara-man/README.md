[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=vara-man/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/vara_man_io)

# Vara Man

### 🏗️ Building

```sh
cargo b -p "vara-man*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "vara-man*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "vara-man*"
```
