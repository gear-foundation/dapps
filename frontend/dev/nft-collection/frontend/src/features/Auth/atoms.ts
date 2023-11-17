import { atom } from 'jotai';

export const AUTH_TOKEN_ATOM = atom<string | null>('');

export const TESTNET_USERNAME_ATOM = atom('');

export const IS_AUTH_READY_ATOM = atom(false);
