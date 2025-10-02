import { HexString } from '@gear-js/api';

export type ZkProof = {
  pi_a: string[];
  pi_b: string[][];
  pi_c: string[];
};

export type ShuffleResult = {
  deck: ZkTaskDeck;
  proof: ZkProof;
  publicSignals: string[];
};

export type ZkStep = 'SHUFFLE';

export type ZkResultRequest = {
  lobbyAddress: string;
  playerAddress: string;
  step: ZkStep;
  result: {
    SHUFFLE?: ShuffleResult;
  };
};

export type ZkTaskCompressedDeck = {
  X0: string[];
  X1: string[];
  selector: string[];
  delta0: string[];
  delta1: string[];
};

export type ZkTaskDeck = string[][];

export type ZkTaskAggKey = {
  X: string;
  Y: string;
  Z: string;
};

export type ZkTaskShuffle = {
  deck: ZkTaskDeck;
  aggKey: ZkTaskAggKey;
};

export type ECPoint<T = bigint> = {
  X: T;
  Y: T;
  Z: T;
};

export type CipherCard = {
  c0: ECPoint;
  c1: ECPoint;
};

export type ZkTaskData = {
  SHUFFLE?: ZkTaskShuffle;
};

export type ZkTaskResponse = {
  step: ZkStep;
  data: ZkTaskData;
};

export type ZkTaskError = {
  message: string;
};

export type ZkTaskApiResponse = ZkTaskResponse | ZkTaskError;

export type ZkResultResponse = {
  ok: boolean;
};

// WebSocket event types
export type SubscribeToTasksPayload = {
  lobbyAddress: string;
  playerAddress: string;
};

export type SubmitResultPayload = {
  lobbyAddress: string;
  playerAddress: string;
  step: ZkStep;
  result: {
    SHUFFLE?: ShuffleResult;
  };
};

export type UnsubscribeFromTasksPayload = {
  lobbyAddress: string;
  playerAddress: string;
};

export type SubscriptionStatusResponse = {
  subscribed: boolean;
  lobbyAddress: string;
  playerAddress: string;
};

export type ResultProcessedResponse = {
  success: boolean;
  message?: string;
};

export interface GameProgressEvent {
  lobbyAddress: string;
  currentPlayerAddress: string;
  completedTasks: number;
  totalTasks: number;
  currentStep: string;
}

export type WebSocketError = {
  message: string;
  code?: string;
};

export type Rank = 'A' | 'K' | 'Q' | 'J' | '10' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2';

export type CardWithPoint = {
  suit: Suit;
  rank: Rank;
  point: ECPoint;
};

export type Card = {
  suit: Suit;
  rank: Rank;
};

export type RevealedPlayer = [HexString, [globalThis.Card, globalThis.Card]];
