import { HexString } from '@polkadot/util/types';

export interface IGameState {
  botAddress: string;
  games: Array<[HexString, IGameInstance]>;
}

type Status = 'pendingVerificationOfTheMove';
export interface IGameInstance {
  bot_ships: Record<string, string>;
  player_board: string[];
  start_time: number;
  end_time: string | null;
  total_shots: string;
  succesfull_shots: number;
  status: Record<Status, number>;
}

export type IGameResultStatus = 'Player' | 'Bot' | 'Draw';

export type GameMode = 'single' | 'find' | 'create' | null;

export type RenderedShip = {
  length: number;
  degrees: number;
};

export type RenderShips = {
  [key: string]: RenderedShip;
};
