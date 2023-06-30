# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.3.8] - 2023-06-30

### Changed
- Updated Gear dependencies to gstd v0.2.1

## [0.3.7] - 2023-05-23
### Added
- `rust-toolchain.toml` for pinning a Rust version.
### Changed
- Updated Gear dependencies.

## [0.3.6] - 2023-04-26
### Changed
- Pin toolchain

## [0.3.5] - 2023-03-02
### Changed
- Updated `gstd`, `gtest`, `gear-wasm-builder` to the `testnet` branch.

### Added
- Derived the `Clone` trait for NFT's IO types.
## [0.3.4] - 2022-12-12
### Added
- Derived the `Clone` trait for NFT's IO types.

## [0.3.3] - 2022-12-03
### Added
- Derived the `PartialOrd` & `Ord` traits for the `TokenMetadata` struct for use in transaction-based contracts.

## [0.3.2] - 2022-11-22
### Changed
- Updated `gstd`, `gtest`, `gear-wasm-builder` to the `stable` branch.

## [0.3.1] - 2022-10-26
### Changed
- Updated `gstd` to the current commit hash (`#d4552434`).

## [0.3.0] - 2022-09-20
### Added
- `gear-lib-sr25519` library for digital signature verifying.
### Changed
- Project structure reworked to be more consistent.
### Removed
- Dependency on `sp-core` due to problem with building contracts.

## [0.2.0] - 2022-08-24
### Changed
- The `NFTCore` trait now returns I/O structs instead of sending them by `msg::reply()` because that behaviour makes impossible for users to define exact types in metadata with the default trait implementation.

## [0.1.0] - 2022-08-22
### Added
- Initial release.

[Unreleased]: https://github.com/gear-dapps/gear-lib/compare/0.3.8...HEAD
[0.3.8]: https://github.com/gear-dapps/gear-lib/compare/0.3.7...0.3.8
[0.3.7]: https://github.com/gear-dapps/gear-lib/compare/0.3.6...0.3.7
[0.3.6]: https://github.com/gear-dapps/gear-lib/compare/0.3.5...0.3.6
[0.3.5]: https://github.com/gear-dapps/gear-lib/compare/0.3.4...0.3.5
[0.3.4]: https://github.com/gear-dapps/gear-lib/compare/0.3.3...0.3.4
[0.3.3]: https://github.com/gear-dapps/gear-lib/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/gear-dapps/gear-lib/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/gear-dapps/gear-lib/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/gear-dapps/gear-lib/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/gear-dapps/gear-lib/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/gear-dapps/gear-lib/compare/67d2566...0.1.0
