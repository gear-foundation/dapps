import { HexString } from '@polkadot/util/types';

type Session = {
  key: HexString;
  expires: string;
  allowedActions: string[];
};

type State = {
  SessionForTheAccount: Session | null;
};

export type { State };
