# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.3.3] - 2023-05-05
### Changed
- Pin Gear dependencies.

## [0.3.1] - 2023-04-12
### Changed
- Update dependencies.

## [0.3.0] - 2022-12-19
### Added
- The error handling support. Now the contract doesn't panic on every handleable error and gracefully returns a self-describing error. Tests became more reliable.
- A limit of participants to avoid the contract memory overflow.
### Changed
- `BTreeMap` was replaced by `HashMap` for performance reasons.
- The logic of interaction with the FT contract was updated. Now GOC shouldn't get stuck on failed token transfer transactions.
- Updated logic of determining a status of the current game round. There were some bugs when a game round ends without participants.

## [0.2.2] - 2022-12-14
### Changed
- Started time doesn't reset after picking a winner.

## [0.2.1] - 2022-12-14
### Changed
- Partially restored `GOCState` from the `0.1.1` version.
- Winner sets to `ActorId::zero()` after every game (re)start.

## [0.2.0] - 2022-11-12
### Added
- A complete rewrite of a code and tests with a minimal API change.
- The [SFT](https://github.com/gear-dapps/sharded-fungible-token) support.
### Changed
- Renamed the contract from Lottery to Game of chance.

## [0.1.1] - 2022-10-26
### Changed
- Updated fungible token to v0.1.2.
- Updated `gstd`, `gtest`, `gear-wasm-builder` to the current commit hash (`#d4552434`).

## [0.1.0] - 2022-07-27
### Added
- Initial release.

[Unreleased]: https://github.com/gear-dapps/lottery/compare/0.3.1...HEAD
[0.3.1]: https://github.com/gear-dapps/lottery/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/gear-dapps/lottery/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/gear-dapps/lottery/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/gear-dapps/lottery/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/gear-dapps/lottery/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/gear-dapps/lottery/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/gear-dapps/lottery/compare/60d5a8e...0.1.0
