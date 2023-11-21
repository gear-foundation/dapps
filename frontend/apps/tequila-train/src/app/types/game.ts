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

export type DominoNumber = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12';

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
    currentPlayer: string;
    players: HexString[];
    remainingTiles: string[];
    shots: string[];
    startTile: string;
    state: IGamePhase;
    tiles: StateDominoTileType[];
    tileToPlayer: {};
    tracks: StatePlayerTrackType[];
    winner: null | HexString;
  };
  players: IPlayer[];
  isStarted: boolean;
  maybeLimit: string;
};

export type IPlayer = [HexString, string];

export type GameWasmStateResponse = {
  currentPlayer: string;
  players: IPlayer[];
  playersTiles: Array<DominoTileType[]>;
  shotCounters: string[];
  startTile: DominoTileType;
  state: IGamePhase;
  tracks: PlayerTrackType[];
  winner: null | HexString;
};

export type PlayerChoiceType = {
  tile?: DominoTileType;
  tile_id?: string;
  track_id?: string;
  remove_train?: boolean;
};
