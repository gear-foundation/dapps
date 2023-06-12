import { HexString } from '@polkadot/util/types';

type Session = {
  altitude: string;
  weather: string;
  fuelPrice: string;
  payloadValue: string;
  registered: {};
};

type SessionState = {
  name: string;
  owner: HexString;
  participants: {};
  currentSession: Session | null;
  events: {};
  state: string;
  sessionId: string;
};

export type { SessionState };
