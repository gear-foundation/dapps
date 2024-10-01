// CHANGE IT
export type GameStatus = 'import' | 'generate' | 'create' | 'find' | null;

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
