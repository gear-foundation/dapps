export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  BASE_NODES: import.meta.env.VITE_DEFAULT_NODES_URL,
  STAGING_NODES: import.meta.env.VITE_STAGING_NODES_URL,
  GAME_STATE_SOCKET: 'wss://state-machine.vara-network.io',
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_TTT,
};

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  // UNAUTHORIZED: '/not-authorized',
  NOTFOUND: '*',
};
