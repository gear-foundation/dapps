<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-quick-start/escrow">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="GEAR">
  </a>
</p>

# Escrow

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
cargo test
```

... or ...

```shell
make test
```

### Run everything with one command

```shell
make all
```

... or ...

```shell
make
```

## License

The source code is licensed under [GPL v3.0 license](LICENSE).
