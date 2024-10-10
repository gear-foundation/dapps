import { AssetsCount } from './types';

export const assetsCount: AssetsCount = {
  hat: 9,
  head: 5,
  body: 6,
  accessories: 11,
};

export const body_colors = ['#C9B7FE', '#FEC9B7'];
export const back_colors = ['#616161', '#FFD700', '#FF69B4', '#49F2C9'];

export const CHARACTER_ASSETS_PATH = './assets/images/character/';

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
