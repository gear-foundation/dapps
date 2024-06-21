import { atom } from 'jotai';
import { MultipleGameState } from '@/features/game/assets/lib/lib';

export const multiplayerGameAtom = atom<MultipleGameState | undefined | null>(undefined);
export const isGameReadyAtom = atom<boolean>(false);
export const isActiveGameAtom = atom<boolean>(false);
export const playerNameAtom = atom<string>('');
