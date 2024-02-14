import { HexString } from '@polkadot/util/types';

type Strategy = {
  name: string;
  fuelAmount: string;
  payloadAmount: string;
};

type Session = {
  altitude: string;
  weather: string;
  reward: string;
  sessionId: string;
};

type Participant = [HexString, Strategy];

type Results = {
  turns: Turns;
  rankings: Rank[];
};

type Event = {
  participant: HexString;
  deadRound: boolean;
  firstDeadRound: number;
  fuelLeft: string;
  lastAltitude: string;
  payload: string;
};

type Rank = [HexString, string];

type LaunchState = {
  Game: {
    admin: HexString;
    stage: {
      Registration: Participant[];
      Results: Results;
    };
    master: string;
    altitude: string;
    weather: string;
    reward: string;
    sessionId: string;
    bid: string;
  };
};

type TurnParticipant = [
  HexString,
  {
    Alive: {
      fuelLeft: string;
      payloadAmount: string;
    };
  },
];

type Turn = TurnParticipant[];

type Turns = Turn[];

type PlayerStatus = 'Finished' | 'Registered' | null;

type PlayerInfo = {
  PlayerInfo: PlayerStatus;
};

export type { LaunchState, Event, Participant, Turns, Rank, TurnParticipant, Session, PlayerStatus, PlayerInfo };
