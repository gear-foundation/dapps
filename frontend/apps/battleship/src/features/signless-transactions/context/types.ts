import { HexString } from '@polkadot/util/types';
import { KeyringPair$Json, KeyringPair } from '@polkadot/keyring/types';

type Session = {
  key: HexString;
  expires: string;
  allowedActions: string[];
};

type State = {
  SessionForTheAccount: Session | null;
};

type Storage = Record<string, KeyringPair$Json | undefined>;

type Value = {
  pair: KeyringPair | undefined;
  savePair: (pair: KeyringPair, password: string) => void;
  unlockPair: (password: string) => void;
  session: Session | null | undefined;
  isSessionReady: boolean;
};

export type { State, Session, Storage, Value };
