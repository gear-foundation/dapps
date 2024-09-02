import { HexString } from '@polkadot/util/types';
import { SingleGame } from '../../app/utils/sails/lib/lib';

export interface IGameState {
  botAddress: string;
  games: Array<[HexString, SingleGame]>;
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

export type GameType = 'single' | 'multi';
