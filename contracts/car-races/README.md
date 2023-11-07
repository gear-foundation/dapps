[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=vara-man/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/car-races_io)

# Car Races

### 🏗️ Building

```sh
cargo b -p "car-races*"
```
```sh
cargo b -p "car-1*"
```
```sh
cargo b -p "car-2*"
```
```sh
cargo b -p "car-3*"
```
### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "car-races*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "car-races*"
```
