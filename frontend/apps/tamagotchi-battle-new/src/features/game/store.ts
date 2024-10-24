import { atom } from 'jotai';
import { BattleHistory, CharacterStatsFormValues } from './types';
import { Appearance } from '@/app/utils';

const getStorage = <T>(key: string) => ({
  set: (item: T | null) => localStorage.setItem(key, JSON.stringify(item)),
  get: (): T | null => {
    const value = localStorage.getItem(key);
    return value ? JSON.parse(value) : null;
  },
});

const CHARACTER_APPEARANCE_LOCAL_KEY = 'character-appearance';
const CHARACTER_STATS_LOCAL_KEY = 'character-stats';
const WARRIOR_ID_LOCAL_KEY = 'warrior-id';
const BATTLE_HISTORY_LOCAL_KEY = 'battle-history';

export const characterAppearanceStorage = getStorage<Appearance>(CHARACTER_APPEARANCE_LOCAL_KEY);
export const characterStatsStorage = getStorage<CharacterStatsFormValues>(CHARACTER_STATS_LOCAL_KEY);
export const warriorIdStorage = getStorage<`0x${string}`>(WARRIOR_ID_LOCAL_KEY);
export const battleHistoryStorage = getStorage<BattleHistory[]>(BATTLE_HISTORY_LOCAL_KEY);

export const characterAppearanceAtom = atom<Appearance | null>(characterAppearanceStorage.get());
export const battleHistoryAtom = atom<BattleHistory[] | null>(battleHistoryStorage.get());

export const otherPairBattleWatchAtom = atom<number | null>(null);
export const isBattleCanceledAtom = atom<boolean>(false);
