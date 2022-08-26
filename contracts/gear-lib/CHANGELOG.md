# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2022-08-24
### Changed
- The `NFTCore` trait now returns I/O structs instead of sending them by `msg::reply()` because that behaviour makes impossible for users to define exact types in metadata with the default trait implementation.

## [0.1.0] - 2022-08-22
### Added
- Initial release.

[Unreleased]: https://github.com/gear-dapps/gear-lib/compare/0.2.0...HEAD
[0.2.0]: https://github.com/gear-dapps/gear-lib/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/gear-dapps/gear-lib/compare/67d2566...0.1.0
