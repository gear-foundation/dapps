import { ONBOARDING_ROUTE } from '@/features/onboarding';

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_TTT as string,
  VOUCHER_LIMIT: import.meta.env.VITE_VOUCHER_LIMIT as string,
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
  ONBOARDING: ONBOARDING_ROUTE,

  NOTFOUND: '*',
};

export const ALLOWED_SIGNLESS_ACTIONS = ['createNewBattle', 'registration', 'startBattle', 'makeMove'];
