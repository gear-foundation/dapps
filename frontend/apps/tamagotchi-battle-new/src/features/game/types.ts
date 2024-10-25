import { Move, Player } from '@/app/utils';

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
  action: Move | null;
  receivedDamage: number;
  health: number;
  isDodged: boolean;
};

export type BattleHistory = {
  player: BattleHistoryItem;
  opponent: BattleHistoryItem;
};

export type CurrentPlayers = {
  player: Player;
  opponent: Player;
};
