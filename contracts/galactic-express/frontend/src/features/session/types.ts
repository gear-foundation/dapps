import { HexString } from '@polkadot/util/types';

type Strategy = {
  fuel: string;
  payload: string;
};

type Participant = {
  name: string;
  balance: string;
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

type Event = {
  participant: HexString;
  alive: boolean;
  fuelLeft: string;
  lastAltitude: string;
  payload: string;
  halt: string | null;
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

export type { LaunchState, Event };
