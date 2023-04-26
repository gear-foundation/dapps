# Gear library for programs

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]
[![Docs][docs_badge]][docs_href]

[build_badge]: https://img.shields.io/github/workflow/status/gear-dapps/supply-chain/Build
[build_href]: https://github.com/gear-dapps/gear-lib/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: LICENSE

[docs_badge]: https://img.shields.io/badge/docs-online-5023dd
[docs_href]: https://dapps.gear.rs/gear_lib

This library provides standard functions used in the implementation of contracts
- fungible token
- non fungible token
- multitoken
- etc

To use the default implementation you should include the packages into your Cargo.toml file:
```toml
gear-lib = { git = "https://github.com/gear-dapps/gear-lib.git" }
gear-lib-derive = { git = "https://github.com/gear-dapps/gear-lib.git" }
```

```rs
use derive_traits::{NFTStateKeeper, NFTCore, NFTMetaState};
use gear_contract_libraries::non_fungible_token::{nft_core::*, state::*, token::*};
```

## License

The source code is licensed under the [MIT license](LICENSE).
