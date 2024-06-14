import { atom } from 'jotai';
import { GameMode, IGameInstance } from './types';

export const gameAtom = atom<IGameInstance | null | undefined>(undefined);
export const pendingAtom = atom<boolean>(false);
export const isActiveGameAtom = atom<boolean>(false);
export const isGameReadyAtom = atom<boolean>(false);
export const gameModeAtom = atom<GameMode>(null);
export const isLoadingAtom = atom<boolean>(false);
