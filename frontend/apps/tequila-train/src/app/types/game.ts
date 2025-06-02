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
  tiles: StateDominoTileType[];
};

type IPhaseWinners = {
  Winners?: HexString[];
};

type IPhaseOther = Record<'Registration' | 'Playing' | 'Stalled', null>;

export type IGamePhase = Partial<IPhaseWinners & IPhaseOther>;

export type IState = {
  config: {
    timeToMove: string;
  };
  games: [HexString, GameType[]];
  playersToGameCreator: [HexString, HexString][];
};

export type PlayersGame = {
  id: HexString;
  lose: boolean;
};

export type IGameState = {
  currentPlayer: string;
  lastActivityTime: string;
  players: PlayersGame[];
  remainingTiles: string[];
  shots: string[];
  startTile: string;
  state: IGamePhase;
  tiles: StateDominoTileType[];
  tileToPlayer: {};
  tracks: PlayerTrackType[];
  Winner: null | string[];
};

export type PlayerChoiceType = {
  tile?: DominoTileType;
  tile_id?: string;
  track_id?: string;
  remove_train?: boolean;
};

export type GameType = {
  admin: HexString;
  bid: string;
  gameState: IGameState;
  initialPlayers: HexString[];
  isStarted: false;
  state: IGamePhase;
};

export type IGame = {
  Game: [GameType | null, null | string];
};
