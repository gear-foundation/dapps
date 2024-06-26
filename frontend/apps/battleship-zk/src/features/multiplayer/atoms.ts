import { atom } from 'jotai';
import { MultipleGameState } from '@/app/utils/sails/lib/lib';

export const multiplayerGameAtom = atom<MultipleGameState | undefined | null>(undefined);
export const isGameReadyAtom = atom<boolean>(false);
export const isActiveGameAtom = atom<boolean>(false);
export const playerNameAtom = atom<string>('');
