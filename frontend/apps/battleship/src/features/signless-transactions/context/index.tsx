import { Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { useAccount } from '@gear-js/react-hooks';
import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';

import { DEFAULT_VALUES, SIGNLESS_STORAGE_KEY } from './consts';
import { Storage, Value } from './types';
import { useSession } from './hooks';

const SignlessTransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = SignlessTransactionsContext;

type Props = {
  children: ReactNode;
};

function SignlessTransactionsProvider({ children }: Props) {
  const { account } = useAccount();
  const { session, isSessionReady } = useSession();

  const [pair, setPair] = useState<KeyringPair | undefined>();

  const getStorage = () => JSON.parse(localStorage[SIGNLESS_STORAGE_KEY] || '{}') as Storage;

  const unlockPair = (password: string) => {
    if (!account) throw new Error('No account address');

    const pairJson = getStorage()[account.address];
    if (!pairJson) throw new Error('Pair not found');

    const keyring = new Keyring({ type: 'sr25519' });
    const result = keyring.addFromJson(pairJson);

    result.unlock(password);
    setPair(result);
  };

  const savePair = (value: KeyringPair, password: string) => {
    if (!account) throw new Error('No account address');

    const storage = { ...getStorage(), [account.address]: value.toJson(password) };

    localStorage.setItem(SIGNLESS_STORAGE_KEY, JSON.stringify(storage));
    setPair(value);
  };

  useEffect(() => {
    if (!session) setPair(undefined);
  }, [session]);

  useEffect(() => {
    setPair(undefined);
  }, [account]);

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
