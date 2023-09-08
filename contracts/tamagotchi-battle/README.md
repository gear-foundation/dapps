[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=tamagotchi-battle/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/tamagotchi_battle_io)

# Tamagotchi battle

### 🏗️ Building

```sh
cargo b -p "tamagotchi-battle*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "tamagotchi-battle*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "tamagotchi-battle*" -- --include-ignored
```
