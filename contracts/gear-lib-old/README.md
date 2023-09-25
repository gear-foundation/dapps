[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=gear-lib-old/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/gear_lib_old)

# Old Gear library

> This library is planned to be deprecated and replaced by the new [Gear library]. It's still used in many contract interacting with tokens in this repository, but if you want to implement your own token contracts, please use the new [Gear library].

[Gear library]: ../gear-lib

Token primitives, helpers for contracts, and everything else that wasn't included in `gstd`.

### ⚙️ Usage

Include the following line under the `[dependencies]` table in your contract's `Cargo.toml` file:
```toml
gear-lib = { git = "https://github.com/gear-foundation/dapps", tag = "0.3.3" }
gear-lib-derive = { git = "https://github.com/gear-foundation/dapps", tag = "0.3.3" }
```
