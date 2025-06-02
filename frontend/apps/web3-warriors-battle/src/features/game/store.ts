import { atom } from 'jotai';

import { Appearance } from '@/app/utils';

import { BattleHistory, CharacterStatsFormValues, CurrentPlayers } from './types';

const getStorage = <T>(key: string) => ({
  set: (item: T | null) => localStorage.setItem(key, JSON.stringify(item)),
  get: () => {
    const value = localStorage.getItem(key);
    return value ? (JSON.parse(value) as T) : null;
  },
});

const CHARACTER_APPEARANCE_LOCAL_KEY = 'character-appearance';
const CHARACTER_STATS_LOCAL_KEY = 'character-stats';
const WARRIOR_ID_LOCAL_KEY = 'warrior-id';
const BATTLE_HISTORY_LOCAL_KEY = 'battle-history';
const CURRENT_PLAYERS_LOCAL_KEY = 'current-players';

export const characterAppearanceStorage = getStorage<Appearance>(CHARACTER_APPEARANCE_LOCAL_KEY);
export const characterStatsStorage = getStorage<CharacterStatsFormValues>(CHARACTER_STATS_LOCAL_KEY);
export const warriorIdStorage = getStorage<`0x${string}`>(WARRIOR_ID_LOCAL_KEY);
export const battleHistoryStorage = getStorage<BattleHistory[]>(BATTLE_HISTORY_LOCAL_KEY);
export const currentPlayersStorage = getStorage<CurrentPlayers>(CURRENT_PLAYERS_LOCAL_KEY);

export const characterAppearanceAtom = atom<Appearance | null>(characterAppearanceStorage.get());
export const battleHistoryAtom = atom<BattleHistory[] | null>(battleHistoryStorage.get());
export const currentPlayersAtom = atom<CurrentPlayers | null>(currentPlayersStorage.get());

export const otherPairBattleWatchAtom = atom<number | null>(null);
export const isBattleCanceledAtom = atom<boolean>(false);
