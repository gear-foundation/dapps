export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL,
  DNS_NAME: import.meta.env.VITE_DNS_NAME,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_TTT,
};

export const MAX_PLAYERS_COUNT = 50;

export const ROUTES = {
  HOME: '/',
  IMPORT_CHARACTER: '/import-character',
  GENERATE_CHARACTER: '/generate-character',
  CREATE_GAME: '/create',
  FIND_GAME: '/find',
  WAITING: '/waiting',

  GAME: '/game',
  ONBOARDING: '/onboarding',

  NOTFOUND: '*',
};
