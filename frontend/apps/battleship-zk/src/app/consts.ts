import { HexString } from '@gear-js/api';

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  GAME: import.meta.env.VITE_CONTRACT_ADDRESS as HexString,
  ZK_PROOF_BACKEND: import.meta.env.VITE_ZK_PROOF_BACKEND_ADDRESS as HexString,
};

export const ROUTES = {
  HOME: '/',
  GAME: '/game',
  NOTFOUND: '*',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['StartGame', 'Turn'];
