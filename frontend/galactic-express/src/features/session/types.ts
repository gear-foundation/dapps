import { HexString } from '@polkadot/util/types';

type Strategy = {
  fuel: string;
  payload: string;
};

type Participant = [HexString, Strategy];

type Session = {
  altitude: string;
  weather: string;
  reward: string;
  sessionId: string;
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
