[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=rock-paper-scissors/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/rock_paper_scissors_io)

# [Rock Paper Scissors](https://wiki.gear-tech.io/docs/examples/rock-paper-scissors)

### 🏗️ Building

```sh
cargo b -p "rock-paper-scissors*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "rock-paper-scissors*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "rock-paper-scissors*"
```
