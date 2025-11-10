import { atom } from 'jotai';

const AUTH_TOKEN_LOCAL_STORAGE_KEY = 'authToken';

const AUTH_TOKEN_ATOM = atom<string | null>(localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY));

const IS_AUTH_READY_ATOM = atom(false);

export { AUTH_TOKEN_LOCAL_STORAGE_KEY, AUTH_TOKEN_ATOM, IS_AUTH_READY_ATOM };
