#  Tequila Train Game

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]

[build_badge]: https://github.com/gear-dapps/tequila-train/workflows/Build/badge.svg
[build_href]: https://github.com/gear-dapps/tequila-train/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/gear-dapps/tequila-train/blob/master/LICENSE

<!-- Description starts here -->

The tequila train game is quite similar to the Mexican train game but has several differences in rules.

🥃🚂 https://tequila-train.com

<!-- End of description -->

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-dapps/tequila-train/releases).

## Building Locally

### Smart Contract

#### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init-contracts
```

#### 🏗️ Build

```shell
cd contracts
cargo build --release
```

... or ...

```shell
make contracts
```

#### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test-contracts
```

### Frontend

#### Install yarn

```shell
npm install --global yarn
```

#### ⚒️ Install deps

```shell
cd frontend && yarn
```

... or ...

```shell
make init-frontend
```

#### 🏗️ Build

```shell
cd frontend && yarn build
```

... or ...

```shell
make frontend
```

#### 🐱‍💻 Serve

```shell
cd frontend && yarn start
```

... or ...

```shell
make serve
```

## 🚀 Run everything with one command

```shell
make
```

## License

The source code is licensed under the [MIT license](LICENSE).
