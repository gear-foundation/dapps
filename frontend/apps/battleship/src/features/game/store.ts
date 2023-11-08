import { atom } from 'jotai';
import { IGameInstance } from './types';

export const gameAtom = atom<IGameInstance | undefined>(undefined);
export const pendingAtom = atom<boolean>(false);
export const isActiveGameAtom = atom<boolean>(false);
