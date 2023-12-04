import { Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { useAccount } from '@gear-js/react-hooks';
import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';

import { LOCAL_STORAGE_SIGNLESS_PAIR_KEY } from './consts';
import { Session, Storage } from './types';
import { useSession } from './hooks';

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
  const { account } = useAccount();
  const { session, isSessionReady } = useSession();

  const [pair, setPair] = useState<KeyringPair | undefined>();

  const getStorage = () => {
    const storage = localStorage[LOCAL_STORAGE_SIGNLESS_PAIR_KEY];

    return storage ? (JSON.parse(storage) as Storage) : {};
  };

  const getSavedPair = () => {
    if (!account) throw new Error('No account address');

    const pairJson = getStorage()[account.address];
    if (!pairJson) return;

    const keyring = new Keyring({ type: 'sr25519' });
    return keyring.addFromJson(pairJson);
  };

  const unlockPair = (password: string) => {
    const savedPair = getSavedPair();
    savedPair?.unlock(password);

    setPair(savedPair);
  };

  const savePair = (value: KeyringPair, password: string) => {
    if (!account) throw new Error('No account address');

    const pairJson = value.toJson(password);
    const storage = { ...getStorage(), [account.address]: pairJson };

    localStorage.setItem(LOCAL_STORAGE_SIGNLESS_PAIR_KEY, JSON.stringify(storage));
    setPair(value);
  };

  useEffect(() => {
    if (!isSessionReady) return;
    if (!session) return setPair(undefined);

    setPair(getSavedPair());

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isSessionReady, session]);

  const value = useMemo(
    () => ({
      pair,
      savePair,
      unlockPair,
      session,
      isSessionReady,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [pair, session, isSessionReady],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions };
