import { HexString } from '@polkadot/util/types';
import { HALT } from './consts';

type Strategy = {
  fuel: string;
  payload: string;
};

type Participant = [HexString, Strategy];

type Session = {
  altitude: string;
  weather: string;
  fuelPrice: string;
  reward: string;
  sessionId: string;
};

type Halt = (typeof HALT)[keyof typeof HALT];

type Event = {
  participant: HexString;
  deadRound: boolean;
  fuelLeft: string;
  lastAltitude: string;
  payload: string;
  halt: Halt | null;
};

type Rank = [HexString, string];

type LaunchState = {
  admin: HexString;
  isSessionEnded: boolean;
  participants: Participant[];
  turns: Turns;
  rankings: Rank[];
  master: string;
  session: Session;
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

export type { LaunchState, Session, Event, Participant, Turns, Rank, TurnParticipant };
