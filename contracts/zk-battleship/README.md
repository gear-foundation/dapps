[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=zk-battleship/https://github.com/gear-foundation/dapps)

# Zero-knowledge battleship

This project is a battleship game that leverages zero-knowledge (zk) cryptography, enabling players to verify each other's moves without revealing any hidden information about their boards. The game preserves privacy through zk proofs, allowing players to prove the validity of their actions while keeping their board configurations confidential.In the repository, there is also a circom directory containing circuits essential for generating the zk proofs required in the game.

For a more in-depth explanation of the game mechanics, design, and zk proof integration, please visit the project [wiki](https://wiki.vara.network/docs/examples/Gaming/Battleship/zk-battleship).

⚙️ **Note**: The project code is developed using the [Sails](https://github.com/gear-tech/sails) framework.

### 🏗️ Building

```sh
cargo b -r -p "zk-battleship"
```

### ✅ Testing

```sh
cargo t -r -p "zk-battleship-app"
```
