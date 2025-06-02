import { atom } from 'jotai';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  GASLESS_BACKEND: import.meta.env.VITE_BACKEND_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  VOUCHER_LIMIT: import.meta.env.VITE_VOUCHER_LIMIT as string,
};

export const ROUTES = {
  HOME: '/',
  GAME: '/battle',
  TEST: '/test',
  NOTFOUND: '*',
};

export const VOUCHER_MIN_LIMIT = 18;

export const GAS_LIMIT = 150000000000;

export const IS_CREATING_VOUCHER_ATOM = atom<boolean>(false);
export const IS_UPDATING_VOUCHER_ATOM = atom<boolean>(false);
