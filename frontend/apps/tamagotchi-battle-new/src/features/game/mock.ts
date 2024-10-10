import { PlayerState } from './types';

const mockPlayer1: PlayerState = {
  name: 'Player name 1',
  health: 100,
  attack: 30,
  defence: 8,
  dodge: 4,
  playerId: 1,
  action: 'attack',
  isDodged: true,
  recivedDamage: 0,
};

const mockPlayer2: PlayerState = {
  name: 'Player name 2',
  health: 100,
  attack: 10,
  defence: 13,
  dodge: 5,
  playerId: 2,
  action: 'reflect',
  isDodged: false,
  recivedDamage: 0,
};

export { mockPlayer1, mockPlayer2 };
