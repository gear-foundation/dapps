[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=galactic-express/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/galactic_express_io)

# Galactic express

Galactic Express (GalEx) is a 100% on-chain PvE economic game.

Deliver the cargo 📦 to the orbit 🌌 using fuel ⛽️ efficiently.

### 🏗️ Building

```sh
cargo b -p "galactic-express*"
```

### ✅ Testing

```sh
cargo t -p "galactic-express*"
```

## Stages

### 1. Registration

> This stage can be started only by the admin.

Contract generates random risk factors, the payload reward, and the fuel price.

During this stage participants have to specify:
- the amount of fuel they're willing to buy for this session.
- the payload weight.

### 2. Execution

> This stage can be started only by the admin.

Contract executes a session within 1 transaction and creates random events during this based on the risk factor of the session.

### 3. The end game

The main goal is to deliver the cargo to the orbit without fuel surplus. A certain reward multiplier is applied based on the fuel tank level. If a player has lots of remaining fuel the delivery reward decreases.

| Fuel left (%) | Multiplier (x) |
| ------------- | -------------- |
| 0             | 1.7            |
| > 0           | 0.5..1.4       |

## Math

### Fuel burn rate

`Fuel burn rate = Payload / Total rounds`

### Risk factor

Risk types effect the mission probability.

| Type                  | Failure probability (%) |
| --------------------- | ----------------------- |
| 🚫 Engine error       | 3                       |
| 🛤 Trajectory failure | 3                       |
| 🚀 Separation error   | 3                       |
| 🗿 Asteroid           | 10  + `weather effect`  |
| ⛽ Fuel > 80%         | 10  + `weather effect`  |
| 📦 Payload > 80%      | 10  + `weather effect`  |

### Weather

| Type       | Effect |
| ---------- | ------ |
| ☀️ Sunny   | 0      |
| ☁️ Cloudy  | 1      |
| 🌧 Rainy   | 2      |
| 🌩 Stormy  | 3      |
| ⛈ Thunder | 4      |
| 🌪 Tornado | 5      |

## To do
- [ ] Add the commit-reveal scheme for the registration phase.
- [ ] The gas reservation feature for autonomous regular sessions.
- [ ] Add PvP elements.
- [ ] Implement speed formula.
