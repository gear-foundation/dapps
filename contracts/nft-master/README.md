[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=nft-master/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/nft_master_io)

# [NFT master](https://wiki.gear-tech.io/docs/developing-contracts/token-standards/gnft721)

This smart contract serves as a generic router for standardized gNFT (gRC-721) contracts on the blockchain network. Its main function is to provide an approved list of addresses for gNFT smart contracts that have been uploaded to Gear-powered networks. Only smart contract operators can change the approved address list.

In general, this smart contract allows for the modification of an entire gNFT contract logic, addition of new features, and creation of new collections without sacrificing the backward compatibility. This enables a web application to dynamically load the necessary gNFTs with all updates, without requiring any additional frontend code or redeployment.

### 🏗️ Building

```sh
cargo b -p "nft-master*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "nft-master*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "nft-master*" -- --include-ignored
```
