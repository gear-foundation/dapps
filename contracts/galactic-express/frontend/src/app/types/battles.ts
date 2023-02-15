import { HexString } from '@polkadot/util/types';

export type TamagotchiColor = 'Green' | 'Red' | 'Blue' | 'Purple' | 'Orange' | 'Yellow';

export type BattlePlayerType = {
  attributes: number[];
  color: TamagotchiColor;
  defence: number;
  health: number;
  power: number;
  owner: HexString;
  tmgId: HexString;
  name: string;
  dateOfBirth: number;
};

export type BattleCurrentStateVariants = 'Registration' | 'GameIsOn' | 'WaitNextRound' | 'GameIsOver';
export type BattleRoundMoveVariants = 'Defence' | 'Attack';

export type RoundDamageType = [number, number, BattleRoundMoveVariants, BattleRoundMoveVariants];

export type BattleStateResponse = {
  admin: HexString;
  currentWinner: HexString;
  players: Record<HexString, BattlePlayerType>;
  playersIds: HexString[];
  round: {
    moves: BattleRoundMoveVariants[];
    players: HexString[];
    tmgIds: HexString[];
    steps: number;
  };
  currentTurn: number;
  state: BattleCurrentStateVariants;
  tmgStoreId: HexString;
};

export type SessionData = {
  altitude: number;
  fuelPrice: number;
  payloadValue: 93
  registered: Participant
  weather: number;
}

export type EventData = {
  alive: boolean;
  fuelLeft: number;
  halt: any
  lastAltitude: number
  participant: HexString
  payload: number
}

export type ParticipantDataType = {
  fuel: number;
  payload: number;
}

export type Participant = {
  [key: HexString]: ParticipantDataType
}

export enum SessionStatus {
  SESSION_IS_OVER = "SessionIsOver",
  REGISTRATION = "Registration",
  INIT = 'Init'
}

export type LouncheStateResponse = {
  currentSession: SessionData | null;
  events: {
    [key: string]: EventData[]
  };
  name: string;
  owner: HexString;
  participants: Participant;
  sessionId: number;
  state: SessionStatus;
}
