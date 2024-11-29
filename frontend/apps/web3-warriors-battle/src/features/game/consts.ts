import { AssetsCount, PlayerState } from './types';

export const TIME_LEFT_GAP = 1;

export const MAX_HEALTH = 100;
export const assetsCount: AssetsCount = {
  hat: 9,
  head: 5,
  body: 6,
  accessories: 11,
};

export const body_colors = ['#C9B7FE', '#FEC9B7'];
export const back_colors = ['#616161', '#FFD700', '#FF69B4', '#49F2C9'];

export const mockCharacterView = {
  hat_index: 6,
  head_index: 1,
  body_index: 0,
  accessory_index: 2,
  body_color: body_colors[0],
  back_color: back_colors[2],
};

export const mockCharacterView2 = {
  hat_index: 7,
  head_index: 1,
  body_index: 1,
  accessory_index: 3,
  body_color: body_colors[1],
  back_color: back_colors[1],
};

const mockPlayer1: PlayerState = {
  name: 'Player name 1',
  health: 100,
  attack: 30,
  defence: 8,
  dodge: 4,
  action: 'Attack',
  isDodged: true,
  receivedDamage: 0,
};

const mockPlayer2: PlayerState = {
  name: 'Player name 2',
  health: 100,
  attack: 10,
  defence: 13,
  dodge: 5,
  action: 'Reflect',
  isDodged: false,
  receivedDamage: 0,
};

export { mockPlayer1, mockPlayer2 };
