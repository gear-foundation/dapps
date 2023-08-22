# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.12] - 2023-07-04
### Changed
- Update dependencies.
- Pin gear crates to `#946ac47`.

## [0.2.11] - 2023-05-25
### Changed
- Update dependencies.
- Pin gear crates to `#78dfa07`.

## [0.2.10] - 2023-04-26
- Pin gear and substrate crates to concrete revisions

## [0.2.9] - 2023-03-02
### Changed
- Update gclient branch to `testnet`
- Update gear-lib version

## [0.2.8] - 2023-01-10
### Added
- Separate Rentable NFT program in another repository
- Add new meta

## [0.2.7] - 2022-12-17
### Added
- Rentable NFT program.

## [0.2.6] - 2022-12-10
### Changed
- Transaction management logic. Now the contract doesn't return `NFTEvent::TransactionMade` if some transaction was made, and cached `NFTEvent` returns instead.

## [0.2.5] - 2022-12-03
### Changed
- Updated `gear-lib`.

## [0.2.4] - 2022-11-22
### Changed
- Updated `gstd`, `gtest`, `gear-wasm-builder` to the `stable` branch.

## [0.2.3] - 2022-10-26
### Changed
- Updated `gstd`, `gtest`, `gear-wasm-builder` to the current commit hash (`#d4552434`).

## [0.2.2] - 2022-10-18
### Changed
- remove `marketplace` and `on-chain-nft` from shared workspace
- use a temporary `patch` in cargo.toml to build contact

## [0.2.1] - 2022-09-20
### Changed
- `gear-lib` updated to version 0.3.0 due to problems with build.

## [0.2.0] - 2022-08-26
### Changed
- The NFT and OnChainNFT contracts now return their `*Event`s as a response to corresponding `*Action`s instead of opaque `Vec<u8>`.

## [0.1.0] - 2022-07-27
### Added
- Initial release.

[Unreleased]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.12...HEAD
[0.2.12]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.11...0.2.12
[0.2.11]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.10...0.2.11
[0.2.10]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.9...0.2.10
[0.2.9]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.8...0.2.9
[0.2.8]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.7...0.2.8
[0.2.7]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.6...0.2.7
[0.2.6]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.5...0.2.6
[0.2.5]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.4...0.2.5
[0.2.4]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.3...0.2.4
[0.2.3]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/gear-dapps/non-fungible-token/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/gear-dapps/non-fungible-token/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/gear-dapps/non-fungible-token/compare/ee684b1...0.1.0
