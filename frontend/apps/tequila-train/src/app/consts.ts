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
  DNS_API_URL: process.env.REACT_APP_DNS_API_URL as string,
  DNS_NAME: process.env.REACT_APP_DNS_NAME as string,
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
};

export const playerNames = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];
