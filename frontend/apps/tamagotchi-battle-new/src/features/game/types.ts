import { Appearance } from '@/app/utils';
import { HexString } from '@gear-js/api';

export type AssetType = 'head' | 'hat' | 'body' | 'accessories';
export type AssetsCount = Record<AssetType, number>;

export type PlayerStatus = 'defeated' | 'alive';

export type PlayerState = {
  name: string;
  health: number;
  attack: number;
  defence: number;
  dodge: number;
  playerId: number;
  action: 'attack' | 'reflect' | 'ultimate';
  isDodged: boolean;
  recivedDamage: number;
};


// @deprecated
export type GameStatus = 'import' | 'generate' | 'create' | 'find' | null;

export type Character = CharacterStatsFormValues & { appearance: Appearance | null; warriorId: HexString | null };

export type CharacterStatsFormValues = {
  attack: number;
  defence: number;
  dodge: number;
};
