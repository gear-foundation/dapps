import { HexString } from '@polkadot/util/types';
import { HALT } from './consts';

type Strategy = {
  fuel: string;
  payload: string;
};

type Participant = {
  name: string;
  balance: string;
  score: string;
};

type Player<T> = {
  [key: HexString]: T;
};

type Session = {
  altitude: string;
  weather: string;
  fuelPrice: string;
  reward: string;
  registered: Player<[Strategy, Participant]>;
  bet: string | null;
};

type Halt = (typeof HALT)[keyof typeof HALT];

type Event = {
  participant: HexString;
  deadRound: string | null;
  fuelLeft: string;
  lastAltitude: string;
  payload: string;
  halt: Halt | null;
};

type LaunchState = {
  name: string;
  owner: HexString;
  participants: Player<Participant>;
  currentSession: Session | null;
  events: { [key: number]: Event[] };
  state: string;
  sessionId: string;
};

export type { LaunchState, Session, Event };
