import { atom } from 'jotai';
import { SingleGame } from '../../app/utils/sails/lib/lib';

export const singleGameAtom = atom<SingleGame | null | undefined>(undefined);
export const isActiveGameAtom = atom<boolean>(false);
export const isGameReadyAtom = atom<boolean>(false);
