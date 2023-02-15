## Getting started

GASA is a 100% on-chain PvE economic game.

Deliver 📦 cargo to the orbit using ⛽️ fuel efficiently

## Stages
1. Registration.

The stage is initialized by the admin.

Session contract generates random risk factors, payload reward and fuel price.

During the registration stage participants have to specify:
- the amount of fuel they're willing to buy for the exact session;
- payload weight.

2. Execution.

The stage is initialized by the admin.

The game is managed by a session contract that executes the session within 1 block.

Session contract creates random events based on the risk factor of the session.

Fuel burn rate = *strategy.payload / total_rounds*

Participants can review the log using the dApp page [title](https://www.example.com).

## Risk factor

Risk types effect the mission probability.

| Type | Failure probability |
| --- | ----------- |
| 🚫 Engine error | 3% |
| 🛤 Trajectory failure | 3% |
| 🚀 Separation error | 3% |
| 🗿 Asteroid | 10% + weather |
| ⛽ Fuel > 80% | 10% + weather |
| 📦 Payload > 80% | 10% + weather |

Weather affects the probability multipliers.

| Risk | Effect |
| --- | ----------- |
| ☀️ sunny | 0 |
| ☁️ cloudy | 1 |
| 🌧 rainy | 2 |
| 🌩 stormy | 3 |
| ⛈ thunder | 4 |
| 🌪 tornado | 5 |

## End Game
The main goal is to deliver the cargo to orbit without fuel surplus. 
A certain reward multiplier apply based on the fuel tank state.
If the player has lots of remaining fuel the delivery reward decreases.

| Fuel left | Multiplier |
| --- | ----------- |
| Fuel tank = 0 | Session reward * 1.7x |
| Fuel tank > 0 | Session reward * 0.5x..1.4x |

[Leaderboard]() is collecting results of previous rocket launches.

## ToDo
- [ ] Add Commit-reveal-scheme for registration phase;
- [ ] Gas reservation feature for autonomous regular sessions;
- [ ] Add PvP elements for the game;
- [ ] Implement speed formula;
- [ ] Build mission probability calculator for registration page;


