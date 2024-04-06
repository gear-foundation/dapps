import { HexString } from '@polkadot/util/types';
import { KeyringPair$Json, KeyringPair } from '@polkadot/keyring/types';

import { useCreateSession } from '../hooks';
import { IVoucherDetails } from '@gear-js/api';

type Session = {
  key: HexString;
  expires: string;
  allowedActions: string[];
};

type State = {
  SessionForTheAccount: Session | null;
};

type Storage = Record<string, KeyringPair$Json | undefined>;

type SignlessContext = {
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
  voucher: (IVoucherDetails & { id: HexString }) | undefined;
  isLoading: boolean;
  setIsLoading: React.Dispatch<React.SetStateAction<boolean>>;
  isActive: boolean;
  isSessionActive: boolean;
  storageVoucher: (IVoucherDetails & { id: HexString }) | undefined;
  storageVoucherBalance: number;
};

export type { State, Session, Storage, SignlessContext };
