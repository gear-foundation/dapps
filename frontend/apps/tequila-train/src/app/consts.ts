import { HexString } from '@polkadot/util/types';

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  // UNAUTHORIZED: '/not-authorized',
  NOTFOUND: '*',
};

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export const ENV = {
  game: process.env.REACT_APP_CONTRACT_ADDRESS as HexString,
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
};

export const playerNames = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];
