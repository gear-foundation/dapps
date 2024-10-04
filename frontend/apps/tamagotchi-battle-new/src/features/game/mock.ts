import { PlayerState } from './types';

const mockPlayer1: PlayerState = {
  name: 'Player name 1',
  currentHealth: 40,
  attack: 30,
  deffence: 10,
  dodge: 30,
  playerId: 1,
  action: 'attack',
  isDodged: true,
  recivedDamage: 13,
};

const mockPlayer2: PlayerState = {
  name: 'Player name 2',
  currentHealth: 0,
  attack: 10,
  deffence: 13,
  dodge: 5,
  playerId: 2,
  action: 'reflect',
  isDodged: false,
  recivedDamage: 0,
};

export { mockPlayer1, mockPlayer2 };
