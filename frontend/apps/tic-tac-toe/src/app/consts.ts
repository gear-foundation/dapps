export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  BASE_NODES: import.meta.env.VITE_DEFAULT_NODES_URL,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL,
  DNS_NAME: import.meta.env.VITE_DNS_NAME,
  STAGING_NODES: import.meta.env.VITE_STAGING_NODES_URL,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_TTT,
  VOUCHER_LIMIT: import.meta.env.VITE_VOUCHER_LIMIT,
};

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  // UNAUTHORIZED: '/not-authorized',
  NOTFOUND: '*',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['StartGame', 'Move', 'Skip'];
