import { atom } from 'jotai';
import { IGameConfig, IGameCountdown, IGameInstance } from './types';

export const gameAtom = atom<IGameInstance | null | undefined>(undefined);
export const configAtom = atom<IGameConfig | null>(null);
export const pendingAtom = atom<boolean>(false);
export const countdownAtom = atom<IGameCountdown>(undefined);
export const stateChangeLoadingAtom = atom<boolean>(false);
