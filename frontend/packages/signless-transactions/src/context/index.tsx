import { HexString, decodeAddress, generateVoucherId } from '@gear-js/api';
import { useAccount, useBalance } from '@gear-js/react-hooks';
import { Keyring } from '@polkadot/api';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { useCreateSession } from '../hooks';
import { DEFAULT_VALUES, SIGNLESS_STORAGE_KEY } from './consts';
import { Storage, Value } from './types';
import { useSession } from './hooks';

const SignlessTransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = SignlessTransactionsContext;

type Props = {
  programId: HexString;
  metadataSource: string;
  children: ReactNode;
};

function useVoucherBalance(programId: HexString, address: string | undefined) {
  const voucherId = address ? generateVoucherId(decodeAddress(address), programId) : undefined;
  const { balance } = useBalance(voucherId);

  return balance ? balance.toNumber() : 0;
}

function SignlessTransactionsProvider({ metadataSource, programId, children }: Props) {
  const metadata = useProgramMetadata(metadataSource);

  const { account } = useAccount();
  const { session, isSessionReady } = useSession(programId, metadata);

  const [pair, setPair] = useState<KeyringPair | undefined>();

  const getStorage = () => JSON.parse(localStorage[SIGNLESS_STORAGE_KEY] || '{}') as Storage;
  const storagePair = account ? getStorage()[account.address] : undefined;

  const { createSession, deleteSession } = useCreateSession(programId, metadata);
  const voucherBalance = useVoucherBalance(programId, storagePair?.address);

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

  const value = {
    pair,
    storagePair,
    savePair,
    deletePair,
    unlockPair,
    session,
    isSessionReady,
    voucherBalance,
    createSession,
    deleteSession,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions };
