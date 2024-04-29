import { HexString } from '@gear-js/api';

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GAME: import.meta.env.VITE_CONTRACT_ADDRESS as HexString,
  BACK: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  BASE_NODES: import.meta.env.VITE_DEFAULT_NODES_URL,
  STAGING_NODES: import.meta.env.VITE_STAGING_NODES_URL,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_TTT,
};

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  // UNAUTHORIZED: '/not-authorized',
  NOTFOUND: '*',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['StartGame', 'Move', 'Skip'];
