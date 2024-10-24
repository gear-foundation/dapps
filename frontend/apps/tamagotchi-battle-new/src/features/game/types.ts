import { Move } from '@/app/utils';

export type AssetType = 'head' | 'hat' | 'body' | 'accessories';
export type AssetsCount = Record<AssetType, number>;

export type PlayerStatus = 'defeated' | 'alive';

export type PlayerState = {
  name: string;
  health: number;
  attack: number;
  defence: number;
  dodge: number;
  action: Move;
  isDodged: boolean;
  receivedDamage: number;
};

export type CharacterStatsFormValues = {
  attack: number;
  defence: number;
  dodge: number;
};

type BattleHistoryItem = {
  action: Move;
  receivedDamage: number;
  health: number;
  isDodged: boolean;
};

export type BattleHistory = {
  player: BattleHistoryItem;
  opponent: BattleHistoryItem;
};
