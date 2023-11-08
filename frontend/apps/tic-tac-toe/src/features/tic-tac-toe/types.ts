export type IGameInstance = {
  board: Cell[];
  botMark: Mark;
  playerMark: Mark;
  lastTime: string;
  gameOver: boolean;
  gameResult: null | IGameResultStatus;
};

export type IGameConfig = {
  addAttributeGas: string; // "40,000,000,000",
  msPerBlock: string; // "3",
  tokensForOwnerGas: string; // "40,000,000,000",
  gasToRemoveGame: string; // "5,000,000,000",
  timeInterval: string; // "20",
  turnDeadlineMs: string; // 120,000 in ms
};

export type IGameResultStatus = 'Player' | 'Bot' | 'Draw';

export enum Mark {
  X = 'X',
  O = 'O',
}

export type Cell = Mark | null;

export type IQueryResponseGame = { Game: IGameInstance | null };
export type IQueryResponseConfig = { Config: IGameConfig | null };

export type IDecodedReplyGame = {
  GameStarted?: {
    game?: IGameInstance;
  };
  MoveMade?: {
    game?: IGameInstance;
  };
};

export type IGameCountdown = { isActive: boolean; value: string } | undefined;
