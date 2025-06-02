import { atom } from 'jotai';

import { SingleGame } from '../../app/utils/sails/lib/lib';

import { GameEndEvent } from './sails/events/use-event-game-end-subscription';

export const singleGameAtom = atom<SingleGame | null | undefined>(undefined);
export const isActiveGameAtom = atom<boolean>(false);
export const isGameReadyAtom = atom<boolean>(false);
export const isGamePengingAtom = atom<boolean>(false);
export const gameEndResultAtom = atom<GameEndEvent | null>(null);
