import { atom } from 'jotai';
import { ADDRESS } from '@/consts';
import { GameState, ICustomNode, MsgIdToGameIdState } from './types';

export const CONTRACT_ADDRESS_ATOM = atom(ADDRESS.CONTRACT);

export const CURRENT_GAME = atom<GameState | null>(null);

export const MSG_TO_GAME_ID = atom<MsgIdToGameIdState | null>(null);

export const nodesAtom = atom<ICustomNode[] | undefined>(undefined);

export const IS_SUBSCRIBED_ATOM = atom<boolean>(false);
