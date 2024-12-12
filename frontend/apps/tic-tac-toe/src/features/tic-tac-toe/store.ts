import { atom } from 'jotai';
import { IGameCountdown } from './types';
import { GameInstance } from '@/app/utils';

export const gameAtom = atom<GameInstance | null | undefined>(undefined);
export const pendingAtom = atom<boolean>(false);
export const countdownAtom = atom<IGameCountdown>(undefined);
export const stateChangeLoadingAtom = atom<boolean>(false);
