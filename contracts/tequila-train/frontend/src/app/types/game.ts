import { HexString } from '@polkadot/util/types';

export type StateDominoNumber =
  | 'Zero'
  | 'One'
  | 'Two'
  | 'Three'
  | 'Four'
  | 'Five'
  | 'Six'
  | 'Seven'
  | 'Eight'
  | 'Nine'
  | 'Ten'
  | 'Eleven'
  | 'Twelve';

export type DominoNumber = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;

export type StateDominoTileType = {
  left: StateDominoNumber;
  right: StateDominoNumber;
};

export type DominoTileType = [DominoNumber, DominoNumber];

export type StatePlayerTrackType = {
  hasTrain: boolean;
  tiles: StateDominoTileType[];
};
export type PlayerTrackType = {
  hasTrain: boolean;
  tiles: DominoTileType[];
};

export type GameStateResponse = {
  currentPlayer: number;
  players: HexString[];
  shots: number[];
  startTile: number;
  tiles: StateDominoTileType[];
  remainingTiles: number[];
  tileToPlayer: {};
  tracks: StatePlayerTrackType[];
  winner: null | HexString;
  state: {
    Winner?: HexString;
    winner?: HexString;
    playing?: null;
  };
};

export type GameWasmStateResponse = {
  currentPlayer: number;
  players: HexString[];
  playersTiles: Array<DominoTileType[]>;
  shotCounters: number[];
  startTile: DominoTileType;
  tracks: PlayerTrackType[];
  winner: null | HexString;
};

export type PlayerChoiceType = {
  tile?: DominoTileType;
  tile_id?: number;
  track_id?: number;
  remove_train?: boolean;
};
