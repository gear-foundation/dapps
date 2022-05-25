<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-academy/ping">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# Ping Pong

An elementary application for familiarity with Gear. You send "PING" and the program responds with "PONG"

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-academy/app/releases/tag/build).

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

The source code is licensed under [GPL v3.0 license](LICENSE).
