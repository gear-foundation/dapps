[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=game-of-chance/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/game_of_chance_io)

# [Game of chance](https://wiki.gear-tech.io/docs/examples/game-of-chance)

### 🏗️ Building

```sh
cargo b -p "game-of-chance*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "game-of-chance*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "game-of-chance*" -- --include-ignored
```
