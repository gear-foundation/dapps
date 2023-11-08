import { atom } from 'jotai';

const AUTH_MESSAGE = 'VARA';

const AUTH_TOKEN_LOCAL_STORAGE_KEY = 'authToken';

const AUTH_TOKEN_ATOM = atom('');
const IS_AUTH_READY_ATOM = atom(false);

export { AUTH_MESSAGE, AUTH_TOKEN_LOCAL_STORAGE_KEY, AUTH_TOKEN_ATOM, IS_AUTH_READY_ATOM };
