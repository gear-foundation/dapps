<p  align="center">
  <a  href="https://gitpod.io/#https://github.com/gear-dapps/dex"  target="_blank">
    <img  src="https://gitpod.io/button/open-in-gitpod.svg"  width="240"  alt="Gitpod">
  </a>
</p>

#  DEX (Decentralized Exchange)

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]

[build_badge]:  https://github.com/gear-dapps/dex/workflows/Build/badge.svg
[build_href]:  https://github.com/gear-dapps/dex/actions/workflows/build.yml

[lic_badge]:  https://img.shields.io/badge/License-MIT-success
[lic_href]:  https://github.com/gear-dapps/app/blob/master/LICENSE

A decentralized exchange (DEX for short) is a peer-to-peer marketplace where transactions occur directly between crypto traders.

##  Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-dapps/dex/releases).

##  Building Locally

###  ⚙️ Install Rust

```shell

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

###  ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

###  🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

###  ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

###  🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

##  License

The source code is licensed under the [MIT license](LICENSE).
