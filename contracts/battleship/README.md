[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=battleship/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/battleship_io)

# Battleship

### 🏗️ Building

```sh
cargo b -p "battleship*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "battleship*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "battleship*"
```
