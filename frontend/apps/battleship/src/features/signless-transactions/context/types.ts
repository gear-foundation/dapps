import { HexString } from '@polkadot/util/types';
import { KeyringPair$Json } from '@polkadot/keyring/types';

type Session = {
  key: HexString;
  expires: string;
  allowedActions: string[];
};

type State = {
  SessionForTheAccount: Session | null;
};

type Storage = Record<string, KeyringPair$Json | undefined>;

export type { State, Session, Storage };
