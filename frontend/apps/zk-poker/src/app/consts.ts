import { HexString } from '@gear-js/api';

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  ZK_POKER_BACKEND: import.meta.env.VITE_ZK_POKER_BACKEND_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  EXPLORER_URL: import.meta.env.VITE_EXPLORER_URL as string,
  FORCED_POKER_FACTORY_PROGRAM_ID: import.meta.env.VITE_POKER_FACTORY_PROGRAM_ID as HexString,
  VOUCHER_LIMIT: Number(import.meta.env.VITE_VOUCHER_LIMIT),
  SIGNLESS_VOUCHER_ISSUE_AMOUNT: Number(import.meta.env.VITE_SIGNLESS_VOUCHER_ISSUE_AMOUNT),
};

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  GAME: '/game/:gameId',
  ONBOARDING: '/onboarding',
  CREATE_GAME: '/create-game',
  ROOMS: '/rooms',
  NOTFOUND: '*',
};

export const SMALL_BLIND = 5;
export const BIG_BLIND = 10;
export const MAX_PLAYERS = 9;
export const SIGNLESS_ALLOWED_ACTIONS = ['AllActions' as const];
