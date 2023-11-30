import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';
import { Keyring } from '@polkadot/api';

import { LOCAL_STORAGE_SIGNLESS_PAIR_KEY } from '../consts';

type Value = {
  password: string;
  setPassword: (value: string) => void;
  setPairJson: (value: KeyringPair$Json) => void;
  pair: KeyringPair | undefined;
};

const DEFAULT_VALUES = {
  password: '',
  setPassword: () => {},
  setPairJson: () => {},
  pair: undefined,
};

const SignlessTransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = SignlessTransactionsContext;

type Props = {
  children: ReactNode;
};

const DEFAULT_PAIR_JSON = localStorage[LOCAL_STORAGE_SIGNLESS_PAIR_KEY]
  ? (JSON.parse(localStorage[LOCAL_STORAGE_SIGNLESS_PAIR_KEY]) as KeyringPair$Json)
  : undefined;

function SignlessTransactionsProvider({ children }: Props) {
  const [pairJson, setPairJson] = useState(DEFAULT_PAIR_JSON);
  const [password, setPassword] = useState('');

  const pair = useMemo(() => {
    if (!password || !pairJson) return;

    try {
      const keyring = new Keyring({ type: 'sr25519' });
      const pair = keyring.addFromJson(pairJson);

      pair.unlock(password);

      return pair;
    } catch (error) {
      console.log('error: ', error);
    }
  }, [pairJson, password]);

  useEffect(() => {
    if (!pairJson) return localStorage.removeItem(LOCAL_STORAGE_SIGNLESS_PAIR_KEY);

    localStorage.setItem(LOCAL_STORAGE_SIGNLESS_PAIR_KEY, JSON.stringify(pairJson));
  }, [pairJson]);

  const value = useMemo(
    () => ({
      password,
      setPassword,
      setPairJson,
      pair,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [password, pair],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions };
