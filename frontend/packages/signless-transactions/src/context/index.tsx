import { HexString } from '@gear-js/api';
import { useAccount, useBalance } from '@gear-js/react-hooks';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { useCreateSession } from '../hooks';
import { DEFAULT_SIGNLESS_CONTEXT, SIGNLESS_STORAGE_KEY } from './consts';
import { SignlessContext } from './types';
import { useSession, useLatestVoucher } from './hooks';
import { getUnlockedPair } from '../utils';
import { getStorage } from './utils';

const SignlessTransactionsContext = createContext<SignlessContext>(DEFAULT_SIGNLESS_CONTEXT);
const { Provider } = SignlessTransactionsContext;

type Props = {
  programId: HexString;
  metadataSource: string;
  children: ReactNode;
};

function SignlessTransactionsProvider({ metadataSource, programId, children }: Props) {
  const { account } = useAccount();

  const metadata = useProgramMetadata(metadataSource);
  const { session, isSessionReady } = useSession(programId, metadata);
  const { createSession, deleteSession } = useCreateSession(programId, metadata);

  const [pair, setPair] = useState<KeyringPair>();
  const voucher = useLatestVoucher(programId, pair?.address);
  const { balance } = useBalance(voucher?.id);
  const voucherBalance = balance ? balance.toNumber() : 0;

  // there's probably a better way to handle storage voucher, since we may not need it in a context
  const [storagePair, setStoragePair] = useState(account ? getStorage()[account.address] : undefined);
  const storageVoucher = useLatestVoucher(programId, storagePair?.address);
  const { balance: _storageVoucherBalance } = useBalance(storageVoucher?.id);
  const storageVoucherBalance = _storageVoucherBalance ? _storageVoucherBalance.toNumber() : 0;

  const [isLoading, setIsLoading] = useState(false);
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

  useEffect(() => {
    if (
      balances?.freeBalance &&
      (Number(getFormattedBalance(balances.freeBalance.toNumber()).value) > 42 || voucherBalance > 0)
    ) {
      setIsAvailable(true);
    } else {
      setIsAvailable(false);
    }
  }, [balances?.freeBalance, storagePair, voucherBalance]);

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
    voucher,
    isLoading,
    setIsLoading,
    isActive,
    isSessionActive,
    storageVoucher,
    storageVoucherBalance,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions, DEFAULT_SIGNLESS_CONTEXT };
export type { SignlessContext };
