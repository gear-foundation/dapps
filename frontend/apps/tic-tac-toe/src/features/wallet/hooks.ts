import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { WALLET, WALLET_ID_LOCAL_STORAGE_KEY } from './consts';
import { IWalletId } from './types';

function useWalletSync() {
  const { account, isAccountReady } = useAccount();
  const { address } = account || {};

  useEffect(() => {
    if (!isAccountReady) return;
    if (!account) return localStorage.removeItem(WALLET_ID_LOCAL_STORAGE_KEY);

    localStorage.setItem(WALLET_ID_LOCAL_STORAGE_KEY, account.meta.source);
  }, [isAccountReady, address, account]);
}

function useWallet() {
  const { accounts } = useAccount();

  const [walletId, setWalletId] = useState(
    (localStorage.getItem(WALLET_ID_LOCAL_STORAGE_KEY) as IWalletId | null) || undefined,
  );

  const wallet = walletId ? WALLET[walletId] : undefined;

  const getWalletAccounts = (id: IWalletId) => accounts?.filter(({ meta }) => meta.source === id);
  const walletAccounts = walletId ? getWalletAccounts(walletId) : undefined;

  const resetWalletId = () => setWalletId(undefined);

  return {
    wallet,
    walletAccounts,
    setWalletId,
    resetWalletId,
    getWalletAccounts,
  };
}

export { useWalletSync, useWallet };
