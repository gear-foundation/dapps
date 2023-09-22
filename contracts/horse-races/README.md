[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=horse-races/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/horce_races_io)

# Horse Races

### 🏗️ Building

```sh
cargo b -p "horse-races*"
```

### ✅ Testing

```sh
cargo t -p "horse-races*"
```

## Overview

The game is a betting on horse racing. The winning horse is determined with random using the oracle. The probability of winning is also affected by additional characteristics of the horse.

The game is based on "races", only one "race" can exist at a time and they alternate. The "races" are managed by a "manager", he can add any number of horses with any characteristics. In addition, the "manager" manages the state of the "races" and can cancel in time or determine the winner. It is important to understand that the "manager" cannot choose any particular participant and cannot influence the result of the "race" in any way!. The "manager" himself cannot take part in bets.

When the "manager" creates a "race", participants are given some time to bid(note that each user is charged a fee for the bet, which was set by the "manager"). When time is up, the "manager" can either cancel the "race" or continue. If the "race" is cancelled, all participants can get their tokens back. If the "race" continues, the contract contacts the oracle, the oracle returns a random value and the contract chooses the winning horse. After that, the tokens of the losing participants are proportionally distributed between the participants who bet on the winning horse. Then the "manager" creates a new "race" and the cycle repeats.
