# Syndote game

Syndote is a Monopoly-like decentralized game that works completely on-chain. Players compete with each other by implementing various playing strategies uploaded as smart-contracts into the network.

Syndote consists of Master contract and Player contracts. Master contract is the main contract that starts and controls the game. Player contracts implement the game strategies that can be unique for each participant of the game. All moves in the game take place automatically, but it is possible to jump to each one individually to analyze the player's strategy.

To launch the game, you need to:
1. ⚒️ Build Master and Player contracts
2. 🏗️ Upload the contracts on chain
3. 🖥️ Build and run user interface

## ⚒️ Build Master and Player contracts

1. Get the source code of [Master contract](https://github.com/gear-tech/syndote-game/tree/master/program/syndote) and [Player contract](https://github.com/gear-tech/syndote-game/tree/master/program/player).
2. Modify Player's contract as you wish to achieve optimal game strategy. 
3. Build contracts:

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

If everything goes well, your working directory should now have a `target` directory that looks like this:

```
target
    ├── CACHEDIR.TAG
    ├── meta.txt
    ├── release
    │   └── ...
    └── wasm32-unknown-unknown
        └── release
            ├── ...
            ├── syndote.wasm      <---- this is built .wasm file
            ├── syndote.opt.wasm  <---- this is optimized .wasm file
            ├── player.wasm       <---- this is built .wasm file
            ├── player.opt.wasm   <---- this is optimized .wasm file
```

### Register players and reserve gas

To run the game you have to deploy the master contract and the players contracts to the network. During initialization the master contract is filled with monopoly card information (cell cost, special cells: jails, lottery, etc).

You have to give enough gas reservation for automatic play. Before each round the master contract checks the amount of gas and if it is not enough it will send a message to the game admin to request for another gas reservation. To make a reservation you have to send to the master contract the following message: 

```rust
GameAction::ReserveGas
```
Currently the single gas reservation amount can be up to 245 000 000 000 since it is not yet possible to make a reservation more than the block gas limit (250 000 000 000). To run the full game you have to make at least 5 reservations.

Then you need to register the contracts of your players. For testing purposes you can upload the same player contract several times. Up to four players or less can be added in the Syndote Master contract.

To register the player you have to send the following message to the Syndote contract:

```rust
GameAction::Register {
    player: ActorId
}
```

After registering players, just start the game via sending the message:

```rust
GameAction::Play
```

If the game is not over, make more reservations and send a message `GameAction::Play` again. 
After the game is over, it's state becomes `Finished` and the admin can restart the game by starting a new player registration:

```rust
GameAction::StartRegistration
```

## 🏗️ Upload contracts on chain

###  Run gear node locally

This is recommended while you are testing and debugging the program.

Here (https://get.gear.rs/) you can find prepared binaries.

```bash
./gear --dev --tmp --unsafe-ws-external --rpc-methods Unsafe --rpc-cors all
```

### Run program in Gear Network

You can deploy contracts using [idea.gear-tech.io](https://idea.gear-tech.io).

More deploy options are available in [Gear Wiki](https://wiki.gear-tech.io/docs/examples/monopoly#run-program-in-gear-network).

## 🖥️ Build and run user interface

To build and run local user interface, use this instruction from [Gear Wiki](https://wiki.gear-tech.io/docs/examples/monopoly#%EF%B8%8F-build-and-run-user-interface).
