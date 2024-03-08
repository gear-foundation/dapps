import { HexString } from '@polkadot/util/types';
import { KeyringPair$Json, KeyringPair } from '@polkadot/keyring/types';

import { useCreateSession } from '../hooks';

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
  storagePair: KeyringPair$Json | undefined;
  savePair: (pair: KeyringPair, password: string) => void;
  deletePair: () => void;
  unlockPair: (password: string) => void;
  session: Session | null | undefined;
  isSessionReady: boolean;
  voucherBalance: number;
  createSession: (...args: Parameters<ReturnType<typeof useCreateSession>['createSession']>) => void;
  deleteSession: (...args: Parameters<ReturnType<typeof useCreateSession>['deleteSession']>) => void;
  updateSession: (...args: Parameters<ReturnType<typeof useCreateSession>['updateSession']>) => void;
  pairVoucherId: `0x${string}` | undefined;
  isLoading: boolean;
  setIsLoading: React.Dispatch<React.SetStateAction<boolean>>;
  isAvailable: boolean;
};

export type { State, Session, Storage, Value };
