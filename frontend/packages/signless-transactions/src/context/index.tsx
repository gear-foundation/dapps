import { GearKeyring, HexString, decodeAddress } from '@gear-js/api';
import { useAccount, useBalance, useBalanceFormat, useDeriveBalancesAll, useVouchers } from '@gear-js/react-hooks';
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
  const decodedAddress = address ? decodeAddress(address) : '';

  const { vouchers } = useVouchers(decodedAddress, programId);

  const voucherId = Object.keys(vouchers || {})[0];
  const { balance } = useBalance(voucherId);

  return balance ? balance.toNumber() : 0;
}

function useVoucherId(programId: HexString, address: string | undefined) {
  const decodedAddress = address ? decodeAddress(address) : '';

  const { vouchers } = useVouchers(decodedAddress, programId);

  const voucherId = Object.keys(vouchers || {})[0];

  return voucherId;
}

function SignlessTransactionsProvider({ metadataSource, programId, children }: Props) {
  const metadata = useProgramMetadata(metadataSource);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isAvailable, setIsAvailable] = useState<boolean>(false);
  const { getFormattedBalance } = useBalanceFormat();
  const { account } = useAccount();
  const { session, isSessionReady } = useSession(programId, metadata);
  const balances = useDeriveBalancesAll(account?.decodedAddress);
  const [pair, setPair] = useState<KeyringPair | undefined>();
  const getStorage = () => JSON.parse(localStorage[SIGNLESS_STORAGE_KEY] || '{}') as Storage;
  const [storagePair, setStoragePair] = useState(account ? getStorage()[account.address] : undefined);

  const { createSession, deleteSession, updateSession } = useCreateSession(programId, metadata);
  const pairVoucherId = useVoucherId(programId, pair?.address) as `0x${string}`;
  const voucherBalance = useVoucherBalance(programId, storagePair?.address);

  const unlockPair = (password: string) => {
    if (!storagePair) throw new Error('Pair not found');

    const result = GearKeyring.fromJson(storagePair, password);

    setPair(result);
  };

  const setPairToStorage = (value: KeyringPair$Json | undefined) => {
    if (!account) throw new Error('No account address');

    const storage = { ...getStorage(), [account.address]: value };

    localStorage.setItem(SIGNLESS_STORAGE_KEY, JSON.stringify(storage));

    setStoragePair(value);
  };

  const savePair = (value: KeyringPair, password: string) => {
    setPairToStorage(value.toJson(password));
    setPair(value);
  };

  const deletePair = () => {
    setPairToStorage(undefined);
    setPair(undefined);
  };

  useEffect(() => {
    if (account) {
      setStoragePair(getStorage()[account.address]);

      return;
    }

    setStoragePair(undefined);
  }, [account?.address]);

  useEffect(() => {
    if (!session) setPair(undefined);
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
    updateSession,
    pairVoucherId,
    isLoading,
    setIsLoading,
    isAvailable,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions };
