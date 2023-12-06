import { Keyring } from '@polkadot/api';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { useAccount, useBalance } from '@gear-js/react-hooks';
import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';

import { DEFAULT_VALUES, SIGNLESS_STORAGE_KEY } from './consts';
import { Storage, Value } from './types';
import { useSession } from './hooks';
import { ADDRESS } from '@/app/consts';
import { decodeAddress, generateVoucherId } from '@gear-js/api';

const SignlessTransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = SignlessTransactionsContext;

type Props = {
  children: ReactNode;
};

function useVoucherBalance(address: string | undefined) {
  const voucherId = address ? generateVoucherId(decodeAddress(address), ADDRESS.GAME) : undefined;
  const { balance } = useBalance(voucherId);

  return balance ? balance.toNumber() : 0;
}

function SignlessTransactionsProvider({ children }: Props) {
  const { account } = useAccount();
  const { session, isSessionReady } = useSession();

  const [pair, setPair] = useState<KeyringPair | undefined>();

  const getStorage = () => JSON.parse(localStorage[SIGNLESS_STORAGE_KEY] || '{}') as Storage;
  const storagePair = account ? getStorage()[account.address] : undefined;

  const voucherBalance = useVoucherBalance(storagePair?.address);

  const unlockPair = (password: string) => {
    if (!storagePair) throw new Error('Pair not found');

    const keyring = new Keyring({ type: 'sr25519' });
    const result = keyring.addFromJson(storagePair);

    result.unlock(password);
    setPair(result);
  };

  const setStoragePair = (value: KeyringPair$Json | undefined) => {
    if (!account) throw new Error('No account address');

    const storage = { ...getStorage(), [account.address]: value };

    localStorage.setItem(SIGNLESS_STORAGE_KEY, JSON.stringify(storage));
  };

  const savePair = (value: KeyringPair, password: string) => {
    setStoragePair(value.toJson(password));
    setPair(value);
  };

  const deletePair = () => {
    setStoragePair(undefined);
    setPair(undefined);
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
      storagePair,
      savePair,
      deletePair,
      unlockPair,
      session,
      isSessionReady,
      voucherBalance,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [pair, session, isSessionReady, voucherBalance],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions };
