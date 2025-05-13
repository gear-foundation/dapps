import { HexString } from '@gear-js/api';

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  ZK_PROOF_BACKEND: import.meta.env.VITE_ZK_PROOF_BACKEND_ADDRESS as HexString,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
};

export const ROUTES = {
  HOME: '/',
  GAME: '/game',
  CREATE_GAME: '/create-game',
  NOTFOUND: '*',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['playSingleGame', 'playMultipleGame'];
