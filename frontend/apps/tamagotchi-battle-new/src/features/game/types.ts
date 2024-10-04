export type AssetType = 'head' | 'hat' | 'body' | 'accessories';
export type AssetsCount = Record<AssetType, number>;

export type PlayerStatus = 'defeated' | 'alive';

export type PlayerState = {
  name: string;
  currentHealth: number;
  attack: number;
  deffence: number;
  dodge: number;
  playerId: number;
  action: 'attack' | 'reflect' | 'ultimate';
  isDodged: boolean;
  recivedDamage: number;
};

// @deprecated
export type GameStatus = 'import' | 'generate' | 'create' | 'find' | null;
