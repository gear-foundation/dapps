import { atom } from 'jotai';
import { GameMode } from './types';
import { SingleGame } from './assets/lib/lib';

export const gameAtom = atom<SingleGame | null | undefined>(undefined);
export const pendingAtom = atom<boolean>(false);
export const isActiveGameAtom = atom<boolean>(false);
export const isGameReadyAtom = atom<boolean>(false);
export const gameModeAtom = atom<GameMode>(null);
export const isLoadingAtom = atom<boolean>(false);
