[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=crowdsale/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/crowdsale_io)

# [Crowdsale (ICO)](https://wiki.gear-tech.io/docs/examples/crowdsale)

### 🏗️ Building

```sh
cargo b -p "crowdsale*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "crowdsale*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "crowdsale*" -- --include-ignored
```
