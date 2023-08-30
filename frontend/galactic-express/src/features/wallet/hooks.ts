import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { LOCAL_STORAGE } from 'consts';
import { WALLET } from './consts';
import { WalletId } from './types';

function useWallet() {
  const { accounts } = useAccount();

  const [walletId, setWalletId] = useState<WalletId | undefined>(localStorage[LOCAL_STORAGE.WALLET]);

  const resetWalletId = () => setWalletId(undefined);

  const getWalletAccounts = (id: WalletId) => accounts.filter(({ meta }) => meta.source === id);

  const saveWallet = () => walletId && localStorage.setItem(LOCAL_STORAGE.WALLET, walletId);

  const removeWallet = () => localStorage.removeItem(LOCAL_STORAGE.WALLET);

  const wallet = walletId && WALLET[walletId];
  const walletAccounts = walletId && getWalletAccounts(walletId);

  return { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet };
}

export { useWallet };
