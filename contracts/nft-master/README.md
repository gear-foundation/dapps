<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-dapps/nft-master" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# NFT Master

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]

[build_badge]: https://img.shields.io/github/actions/workflow/status/gear-dapps/nft-master/build.yml?label=Build
[build_href]: https://github.com/gear-dapps/nft-master/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/gear-dapps/nft-master/blob/master/LICENSE

<!-- Description starts here -->

This smart contract serves as a generic router for standardized gNFT (gRC-721) contracts on the blockchain network. Its main function is to provide an approved list of addresses for gNFT smart contracts that have been uploaded to Gear-powered networks. Only smart contract operators can change the approved address list.

In general, this smart contract allows for the modification of the entire gNFT contract logic, addition of new features, and creation of new collections without sacrificing backward compatibility. This enables the web application to dynamically load the necessary gNFTs with all updates, without requiring any additional frontend code or redeployment.

<!-- End of description -->

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-dapps/nft-master/releases).

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

## License

The source code is licensed under the [MIT license](LICENSE).
