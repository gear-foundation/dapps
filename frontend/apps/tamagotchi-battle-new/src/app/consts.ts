export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL,
  DNS_NAME: import.meta.env.VITE_DNS_NAME,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_TTT,
};

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  GAME: '/game',
  NOTFOUND: '*',
};
