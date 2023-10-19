import { atom } from 'jotai';

const AUTH_API_ADDRESS = process.env.REACT_APP_AUTH_API_ADDRESS as string;

const AUTH_MESSAGE = 'VARA';

const AUTH_TOKEN_LOCAL_STORAGE_KEY = 'authToken';

const AUTH_TOKEN_ATOM = atom<string | null>(localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY));

const IS_AUTH_READY_ATOM = atom(false);

export { AUTH_API_ADDRESS, AUTH_MESSAGE, AUTH_TOKEN_LOCAL_STORAGE_KEY, AUTH_TOKEN_ATOM, IS_AUTH_READY_ATOM };
