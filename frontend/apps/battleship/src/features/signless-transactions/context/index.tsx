import { HexString } from '@gear-js/api';
import { useAccount, useBalanceFormat, useDeriveBalancesAll } from '@gear-js/react-hooks';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { useCreateSession } from '../hooks';
import { DEFAULT_SIGNLESS_CONTEXT, SIGNLESS_STORAGE_KEY } from './consts';
import { Storage, SignlessContext } from './types';
import { useSession, useVoucherBalance, useVoucherId } from './hooks';
import { getUnlockedPair } from '../utils';

const SignlessTransactionsContext = createContext<SignlessContext>(DEFAULT_SIGNLESS_CONTEXT);
const { Provider } = SignlessTransactionsContext;

type Props = {
  programId: HexString;
  metadataSource: string;
  children: ReactNode;
};

function SignlessTransactionsProvider({ metadataSource, programId, children }: Props) {
  const { account } = useAccount();
  const balances = useDeriveBalancesAll(account?.address);

  const { getFormattedBalance } = useBalanceFormat();

  const metadata = useProgramMetadata(metadataSource);
  const { session, isSessionReady } = useSession(programId, metadata);
  const { createSession, deleteSession, updateSession } = useCreateSession(programId, metadata);

  const [pair, setPair] = useState<KeyringPair>();
  const pairVoucherId = useVoucherId(programId, pair?.address);
  const voucherBalance = useVoucherBalance(programId, pair?.address);

  const getStorage = () => JSON.parse(localStorage[SIGNLESS_STORAGE_KEY] || '{}') as Storage;
  const [storagePair, setStoragePair] = useState(account ? getStorage()[account.address] : undefined);

  const [isLoading, setIsLoading] = useState(false);
  const [isAvailable, setIsAvailable] = useState(false);
  const isActive = Boolean(pair);
  const isSessionActive = Boolean(session);

  const unlockPair = (password: string) => {
    if (!storagePair) throw new Error('Pair not found');

    const result = getUnlockedPair(storagePair, password);

    setPair(result);
  };

  const setPairToStorage = (value: KeyringPair$Json | undefined) => {
    if (!account) throw new Error('No account address');

    const storage = { ...getStorage(), [account.address]: value };

    localStorage.setItem(SIGNLESS_STORAGE_KEY, JSON.stringify(storage));
    setStoragePair(value);
  };

  useEffect(() => {
    if (!account) return setStoragePair(undefined);

    setStoragePair(getStorage()[account.address]);
  }, [account]);

  useEffect(() => {
    if (!balances) return;
    const { freeBalance } = balances;

    const result = balances
      ? Number(getFormattedBalance(freeBalance.toNumber()).value) > 42 || voucherBalance > 0
      : false;

    setIsAvailable(result);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [balances, voucherBalance]);

  const savePair = (value: KeyringPair, password: string) => {
    setPairToStorage(value.toJson(password));
    setPair(value);
  };

  const deletePair = () => {
    setPairToStorage(undefined);
    setPair(undefined);
  };

  useEffect(() => {
    if (session) return;

    setPair(undefined);
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
    updateSession,
    pairVoucherId,
    isLoading,
    setIsLoading,
    isAvailable,
    isActive,
    isSessionActive,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions, DEFAULT_SIGNLESS_CONTEXT };
export type { SignlessContext };
