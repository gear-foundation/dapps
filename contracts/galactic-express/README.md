# Galactic Express

Galactic Express (GalEx) is a 100% on-chain PvE economic game.

Deliver the cargo 📦 to the orbit 🌌 using fuel ⛽️ efficiently.

A detailed description of the project can be found on the [wiki](https://wiki.vara.network/docs/examples/Gaming/galactic-express).

⚙️ **Note**: The project code is developed using the [Sails](https://github.com/gear-tech/sails) framework.

### 🏗️ Building

```sh
cargo b -r -p "galactic-express"
```

### ✅ Testing

```sh
cargo t -r -p "galactic-express"
```

## Stages

### 1. Registration

Contract generates random risk factors, the payload reward, and the fuel price.

During this stage participants have to specify:
- an amount of fuel they're willing to buy for this session.
- a payload weight.

### 2. Execution

> This stage can be started only by the admin.

Contract executes a session within 1 transaction and creates random events during it based on risk factors of the session.

### 3. The end game

The main goal is to deliver a cargo to the orbit without the fuel surplus/shortage and with the bypass of fatal events. If a player has lots of remaining fuel, a delivery reward increases.

## Math

### Fuel burn rate

`= (Payload weight + 2 * Weather factor) / Turns)`

### Risk factor

Risk types effect the mission probability.

|                                                        Type                 | Failure probability (%) |
| --------------------------------------------------------------------------- | ----------------------- |
|                                                       🚫 Engine failure     | 3                       |
|                                                       🚀 Separation failure | 5 + Weather factor      |
|                                                       🗿 Asteroid collision | 10 + Weather factor     |
| If a fuel amount > (80 - 2 * Weather factor)%,<br>    ⛽ Fuel overload      | 10                      |
| If a payload amount > (80 - 2 * Weather factor)%,<br> 📦 Payload overload   | 10                      |

### Weather

| Type       | Effect |
| ---------- | ------ |
| ☀️ Clear   | 0      |
| ☁️ Cloudy  | 1      |
| 🌧 Rainy   | 2      |
| 🌩 Stormy  | 3      |
| ⛈ Thunder | 4      |
| 🌪 Tornado | 5      |

## To do
- [ ] Add the commit-reveal scheme for the registration phase.
- [ ] Add PvP elements.
- [ ] Implement the speed formula.
