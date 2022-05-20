<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-academy/erc1155">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="GEAR">
  </a>
</p>

# ERC-1155

## Description

Tokens standards like ERC-20 and ERC-721 require a separate contract to be deployed for each token type or collection. This places a lot of redundant bytecode on the Ethereum blockchain and limits certain functionality by the nature of separating each token contract into its own permissioned address. With the rise of blockchain games and platforms like Enjin Coin, game developers may be creating thousands of token types, and a new type of token standard is needed to support them. However, ERC-1155 is not specific to games and many other applications can benefit from this flexibility.

New functionality is possible with this design such as transferring multiple token types at once, saving on transaction costs. Trading (escrow / atomic swaps) of multiple tokens can be built on top of this standard and it removes the need to “approve” individual token contracts separately. It is also easy to describe and mix multiple fungible or non-fungible token types in a single contract.

## Interface

### Events

```rust
// `TransferSingle` MUST emit when tokens are transferred, including zero value transfers as well as minting or burning
TransferSingle {
    operator: ActorId,
    from: ActorId,
    to: ActorId,
    id: u128,
    amount: u128,
}

// `TransferBatch` MUST emit when tokens are transferred, including zero value transfers as well as minting or burning
TransferBatch {
    operator: ActorId,
    from: ActorId,
    to: ActorId,
    ids: Vec<u128>,
    values: Vec<u128>,
}

// MUST emit when approval for a second party/operator address to manage all tokens for an owner address is enabled or disabled (absence of an event assumes disabled)
ApprovalForAll {
    owner: ActorId,
    operator: ActorId,
    approved: bool,
}
```

### Functions

```rust
// Get the balance of an account's tokens
fn balance_of(&self, account: &ActorId, id: &u128) -> u128;

// Get the balance of multiple account/token pairs
fn balance_of_batch(&self, accounts: &[ActorId], ids: &[u128]) -> Vec<BalanceOfBatchReply>;

// Enable or disable approval for a third party ("operator") to manage all of the caller's tokens, and MUST emit the ApprovalForAll event
fn set_approval_for_all(&mut self, operator: &ActorId, approved: bool);

// Queries the approval status of an operator for a given owner
fn is_approved_for_all(&self, account: &ActorId, operator: &ActorId) -> bool;

// Transfers amount of tokens from address to other address, and MUST emit the TransferSingle event
fn safe_transfer_from(&mut self, from: &ActorId, to: &ActorId, id: &u128, amount: u128);

// Transfers  multiple type amount of tokens from address to other address, and MUST emit the TransferBatch event
fn safe_batch_transfer_from(&mut self, from: &ActorId, to: &ActorId, ids: &[u128], amounts: &[u128]);
```

### Init Config

```rust
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}
```

### `Action` Structure

```rust
#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Action {
    Mint(ActorId, u128, u128),
    BalanceOf(ActorId, u128),
    BalanceOfBatch(Vec<ActorId>, Vec<u128>),
    MintBatch(ActorId, Vec<u128>, Vec<u128>),
    SafeTransferFrom(ActorId, ActorId, u128, u128),
    SafeBatchTransferFrom(ActorId, ActorId, Vec<u128>, Vec<u128>),
    SetApprovalForAll(ActorId, bool),
    IsApprovedForAll(ActorId, ActorId),
    BurnBatch(Vec<u128>, Vec<u128>),
    OwnerOf(u128),
    OwnerOfBatch(Vec<u128>),
}
```

### `Event` Structure

```rust
pub enum Event {
    TransferSingle(TransferSingleReply),
    Balance(u128),
    BalanceOfBatch(Vec<BalanceOfBatchReply>),
    MintOfBatch(Vec<BalanceOfBatchReply>),
    TransferBatch {
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        ids: Vec<u128>,
        values: Vec<u128>,
    },
    ApprovalForAll {
        owner: ActorId,
        operator: ActorId,
        approved: bool,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct TransferSingleReply {
    pub operator: ActorId,
    pub from: ActorId,
    pub to: ActorId,
    pub id: u128,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BalanceOfBatchReply {
    pub account: ActorId,
    pub id: u128,
    pub amount: u128,
}
```

## Ref

https://docs.openzeppelin.com/contracts/4.x/api/token/erc1155


## Using

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

### ✍️ Edit the program

Open [`src/lib.rs`](src/lib.rs) and address all `TODO`s there.

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
