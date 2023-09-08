[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=student-nft/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts-docs.yml?logo=rust&label=docs)](https://dapps.gear.rs/student_nft_io)

# Student NFT

A special NFT implementation for Gear Academy students. Learn, react, upvote, comment!

### 🏗️ Building

```sh
cargo b -p "student-nft*"
```

### ✅ Testing

Run only `gtest` tests:
```sh
cargo t -p "student-nft*"
```

Run `gtest` & `gclient` tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "student-nft*" -- --include-ignored
```
