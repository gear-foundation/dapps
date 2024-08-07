import { atom } from 'jotai';
import { GameState, MsgIdToGameIdState } from './types';

export const CURRENT_GAME = atom<GameState | null>(null);

export const IS_CURRENT_GAME_READ_ATOM = atom<boolean>(false);

export const MSG_TO_GAME_ID = atom<MsgIdToGameIdState | null>(null);

export const IS_SUBSCRIBED_ATOM = atom<boolean>(false);
