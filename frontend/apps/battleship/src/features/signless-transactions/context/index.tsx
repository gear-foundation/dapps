import { KeyringPair } from '@polkadot/keyring/types';
import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';

import { LOCAL_STORAGE_SIGNLESS_PAIR_KEY } from './consts';
import { Session } from './types';
import { useSession } from './hooks';
import { getSavedPair } from './utils';

type Value = {
  pair: KeyringPair | undefined;
  savePair: (pair: KeyringPair, password: string) => void;
  unlockPair: (password: string) => void;
  session: Session | null | undefined;
  isSessionReady: boolean;
};

const DEFAULT_VALUES = {
  pair: undefined,
  savePair: () => {},
  unlockPair: () => {},
  session: undefined,
  isSessionReady: false,
};

const SignlessTransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = SignlessTransactionsContext;

type Props = {
  children: ReactNode;
};

function SignlessTransactionsProvider({ children }: Props) {
  const { session, isSessionReady } = useSession();

  const [pair, setPair] = useState<KeyringPair | undefined>();

  const unlockPair = (password: string) => {
    const savedPair = getSavedPair();
    savedPair?.unlock(password);

    setPair(savedPair);
  };

  const savePair = (value: KeyringPair, password: string) => {
    const pairJson = value.toJson(password);
    localStorage.setItem(LOCAL_STORAGE_SIGNLESS_PAIR_KEY, JSON.stringify(pairJson));

    setPair(value);
  };

  useEffect(() => {
    if (!isSessionReady) return;
    if (!session) return setPair(undefined);

    setPair(getSavedPair());
  }, [isSessionReady, session]);

  const value = useMemo(
    () => ({
      pair,
      savePair,
      unlockPair,
      session,
      isSessionReady,
    }),
    [pair, session, isSessionReady],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions };
