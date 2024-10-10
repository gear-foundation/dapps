import { atom } from 'jotai';
import { Character, GameStatus } from './types';

const CHARACTER_LOCAL_KEY = 'character';

export const characterStorage = {
  set: (character: Character) => localStorage.setItem(CHARACTER_LOCAL_KEY, JSON.stringify(character)),
  get: (): Character | null => {
    const value = localStorage.getItem(CHARACTER_LOCAL_KEY);
    return value ? JSON.parse(value) : null;
  },
};

// ! TODO
export const gameStatusAtom = atom<GameStatus>(null);

export const characterAtom = atom<Character | null>(characterStorage.get());
