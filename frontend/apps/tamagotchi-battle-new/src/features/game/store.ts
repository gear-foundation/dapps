import { atom } from 'jotai';
import { BattleHistory, Character } from './types';

const getStorage = <T>(key: string) => ({
  set: (item: T | null) => localStorage.setItem(key, JSON.stringify(item)),
  get: (): T | null => {
    const value = localStorage.getItem(key);
    return value ? JSON.parse(value) : null;
  },
});

const CHARACTER_LOCAL_KEY = 'character';
const BATTLE_HISTORY_LOCAL_KEY = 'battle-history';

export const characterStorage = getStorage<Character>(CHARACTER_LOCAL_KEY);
export const battleHistoryStorage = getStorage<BattleHistory[]>(BATTLE_HISTORY_LOCAL_KEY);

export const characterAtom = atom<Character | null>(characterStorage.get());
export const battleHistoryAtom = atom<BattleHistory[] | null>(battleHistoryStorage.get());
