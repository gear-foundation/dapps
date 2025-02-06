import { atom } from 'jotai';

import { GameMode } from './types';

export const pendingAtom = atom<boolean>(false);
export const gameModeAtom = atom<GameMode>(null);
