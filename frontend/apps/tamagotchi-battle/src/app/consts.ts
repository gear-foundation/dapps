import { atom } from 'jotai';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export const ENV = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  BACK: process.env.REACT_APP_BACKEND_ADDRESS as string,
};

export const ROUTES = {
  HOME: '/',
  GAME: '/battle',
  TEST: '/test',
  NOTFOUND: '*',
};

export const VOUCHER_MIN_LIMIT = 18;

export const GAS_LIMIT = 250000000000;

export const IS_CREATING_VOUCHER_ATOM = atom<boolean>(false);
export const IS_UPDATING_VOUCHER_ATOM = atom<boolean>(false);
