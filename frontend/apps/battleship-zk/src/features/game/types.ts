import { HexString } from '@polkadot/util/types';
import { SingleGame } from './assets/lib/lib';

export interface IGameState {
  botAddress: string;
  games: Array<[HexString, SingleGame]>;
}

type Status = 'pendingVerificationOfTheMove';

export type IGameResultStatus = 'Player' | 'Bot' | 'Draw';

export type GameMode = 'single' | 'find' | 'create' | null;

export type RenderedShip = {
  length: number;
  degrees: number;
};

export type RenderShips = {
  [key: string]: RenderedShip;
};
