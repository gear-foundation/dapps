import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { LOCAL_STORAGE_WALLET_ID_KEY, WALLET } from './consts';
import { WalletId } from './types';

function useWallet() {
  const { accounts } = useAccount();

  const [walletId, setWalletId] = useState<WalletId | undefined>(localStorage[LOCAL_STORAGE_WALLET_ID_KEY]);

  const resetWalletId = () => setWalletId(undefined);

  const getWalletAccounts = (id: WalletId) => accounts?.filter(({ meta }) => meta.source === id);

  const saveWallet = () => walletId && localStorage.setItem(LOCAL_STORAGE_WALLET_ID_KEY, walletId);

  const removeWallet = () => localStorage.removeItem(LOCAL_STORAGE_WALLET_ID_KEY);

  const wallet = walletId && WALLET[walletId];
  const walletAccounts = walletId && getWalletAccounts(walletId);

  return { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet };
}

export { useWallet };
