import { atom } from 'jotai';

import { MultipleGameState } from '@/app/utils/sails/lib/lib';

import { GameEndEvent } from './sails/events/use-event-game-end-subscription';

export const multiplayerGameAtom = atom<MultipleGameState | undefined | null>(undefined);
export const isGameReadyAtom = atom<boolean>(false);
export const isActiveGameAtom = atom<boolean>(false);
export const playerNameAtom = atom<string>('');
export const gameEndResultAtom = atom<GameEndEvent | null>(null);
