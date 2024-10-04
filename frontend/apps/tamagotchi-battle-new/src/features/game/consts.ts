import { AssetsCount } from './types';

export const assetsCount: AssetsCount = {
  hat: 9,
  head: 5,
  body: 6,
  accessories: 11,
};

export const bodyColors = ['#C9B7FE', '#FEC9B7'];
export const backColors = ['#616161', '#FFD700', '#FF69B4', '#49F2C9'];

export const CHARACTER_ASSETS_PATH = './assets/images/character/';

export const mockCharacterView = {
  hatIndex: 6,
  headIndex: 1,
  bodyIndex: 0,
  accessoryIndex: 2,
  bodyColor: bodyColors[0],
  backColor: backColors[2],
};

export const mockCharacterView2 = {
  hatIndex: 7,
  headIndex: 1,
  bodyIndex: 1,
  accessoryIndex: 3,
  bodyColor: bodyColors[1],
  backColor: backColors[1],
};
