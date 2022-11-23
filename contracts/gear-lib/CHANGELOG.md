# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/gear-dapps/gear-lib/compare/0.3.2...HEAD
[0.3.2]: https://github.com/gear-dapps/gear-lib/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/gear-dapps/gear-lib/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/gear-dapps/gear-lib/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/gear-dapps/gear-lib/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/gear-dapps/gear-lib/compare/67d2566...0.1.0
