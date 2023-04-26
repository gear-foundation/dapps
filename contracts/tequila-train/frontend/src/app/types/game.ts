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

type IPhaseWinner = Record<'winner', IPlayer>;

type IPhaseOther = Record<'registration' | 'playing' | 'stalled', null>;

export type IGamePhase = Partial<IPhaseWinner & IPhaseOther>;

export type IGameState = {
  gameState: {
    currentPlayer: number;
    players: HexString[];
    remainingTiles: number[];
    shots: number[];
    startTile: number;
    state: IGamePhase;
    tiles: StateDominoTileType[];
    tileToPlayer: {};
    tracks: StatePlayerTrackType[];
    winner: null | HexString;
  };
  players: IPlayer[];
  isStarted: boolean;
  maybeLimit: number;
};

export type IPlayer = [HexString, string];

export type GameWasmStateResponse = {
  currentPlayer: number;
  players: IPlayer[];
  playersTiles: Array<DominoTileType[]>;
  shotCounters: number[];
  startTile: DominoTileType;
  state: IGamePhase;
  tracks: PlayerTrackType[];
  winner: null | HexString;
};

export type PlayerChoiceType = {
  tile?: DominoTileType;
  tile_id?: number;
  track_id?: number;
  remove_train?: boolean;
};
