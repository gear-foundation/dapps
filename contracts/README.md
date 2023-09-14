[![Open in Gitpod]](https://gitpod.io/#https://github.com/gear-foundation/dapps)
[![Docs]](https://dapps.gear.rs/ping_io)
[![CI](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-build.yml?logo=github&label=CI)](https://github.com/gear-foundation/dapps/actions/workflows/contracts-build.yml)

# Gear Ecosystem Contracts

This workspace contains reference implementations of standardized contracts & examples of contracts. More information about them & setting the environment up can be found on [Gear Wiki](https://wiki.gear-tech.io/docs/examples/prerequisites).

## Supported operating systems

**Linux**<br>
The building, and the test passing is fully supported & checked by CI.

**macOS**<br>
The building, and the test passing should work, but the workspace doesn't support the node downloading by `cargo xtask`. Not checked by CI.

**Windows**<br>
Not supported.

## Usage

The workspace consists of mainly contracts & a few libraries. Some of them have a clickable title in `README.md`, it means they have an article on [Gear Wiki](https://wiki.gear-tech.io).

Above the title, there are clickable badges:
- ![Open in Gitpod] opens a Gitpod workspace in your browser with the set up environment to play with code.
- ![Docs] opens the portal with generated documentation from the workspace.

### 🏗️ Build all contract & states

```sh
cargo b
```

### ✅ Build & run tests

Run all tests, except `gclient` ones:
```sh
cargo t -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t
```

### 🚀 Run CI locally (should be done before a commit)
```sh
cargo xtask ci
```

## Versioning & backwards compatibility

The workspace has the same version as the latest stable release of the [Gear runtime](https://github.com/gear-tech/gear), so there's no backwards compatibility, and it's recommended to be ready for any breaking change & always use the latest version.

[Open in Gitpod]: https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod
[Docs]: https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-build.yml?logo=rust&label=docs
