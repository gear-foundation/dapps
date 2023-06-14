import { HexString } from '@polkadot/util/types';

type Strategy = {
  fuel: string;
  payload: string;
};

type Participant = {
  name: string;
  balance: string;
};

type Session = {
  altitude: string;
  weather: string;
  fuelPrice: string;
  reward: string;
  registered: { [key: HexString]: [Strategy, Participant] };
  bet: string;
};

type LaunchState = {
  name: string;
  owner: HexString;
  participants: {};
  currentSession: Session | null;
  events: {};
  state: string;
  sessionId: string;
};

export type { LaunchState };
