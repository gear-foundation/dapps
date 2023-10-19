[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=dutch-auction/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/dutch_auction_io)

# [Dutch auction](https://wiki.gear-tech.io/docs/examples/DeFi/dutch-auction)

### 🏗️ Building

```sh
cargo b -p "dutch-auction*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "dutch-auction*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "dutch-auction*"
```
