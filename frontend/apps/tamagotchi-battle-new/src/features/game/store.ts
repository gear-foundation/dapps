import { atom } from 'jotai';
import { GameStatus } from './types';

export const gameStatusAtom = atom<GameStatus>(null);
