import { atom } from 'jotai';

export const CURRENT_GAME_ATOM = atom<string>('');

export const PLAYER_NAME_ATOM = atom<string | null>(null);

export const PLAYER_INITIAL_STATUS_ATOM = atom<'Finished' | 'Registered' | null>(null);

export const IS_LOADING = atom<boolean>(false);
