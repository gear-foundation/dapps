import { HexString } from '@polkadot/util/types';

export type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];

export type ArrayElement<ArrayType extends readonly unknown[]> = ArrayType extends readonly (infer ElementType)[]
  ? ElementType
  : never;

export type Handler = (event: Event) => void;

export type GamesState = [string, GameState][];

export interface Car {
  balance: string;
  penalty: string;
  position: string;
  speed: string;
  roundResult: string | null;
  carActions: [];
}

export interface Cars {
  [key: string]: Car;
}

export type DecodedReplyItem = [HexString, string, string];

export type DecodedReply = {
  cars: DecodedReplyItem[];
  result: 'Win' | 'Draw' | 'Lose';
};

export interface GameState {
  cars: Cars;
  carIds: string[];
  currentTurn: string;
  state: 'PlayerAction' | 'Finished';
  result: 'Win' | 'Draw' | 'Lose' | null;
  currentRound: string;
}

export type CurrentGameState = { state: { Game: GameState }; isStateRead: boolean; error: any };

export interface MsgIdToGameIdState {}

export interface ConfigState {
  leaderboardContract: any;
  ftContract: any;
  nftMembershipGuard: any;
  tokensOnWin: string;
  tokensOnDraw: string;
  tokensOnLose: string;
}

export type StrategyIds = {
  StrategyIds: string[];
};

export type Game = { Game: GameState };

export type Config = {
  Config: ConfigState;
};

export type AllGames = {
  AllGames: GamesState;
};

export interface ProgramState {
  admin: HexString;
  strategyIds: string[];
  games: GamesState;
  msgIdToGameId: MsgIdToGameIdState;
  config: ConfigState;
}

export interface ProgramStateRes<T> {
  state?: T;
  isStateRead: Boolean;
  error: string;
}

export type INode = {
  address: string;
  isCustom: boolean;
  icon?: string;
};

export type INodeSection = {
  caption: string;
  nodes: INode[];
};

export type ICustomNode = INode & {
  caption: string;
};

export type ContractError = {
  message?: string;
};
