import { useEffect, useState } from 'react';
import { useAlert, useAccount } from '@gear-js/react-hooks';
import { Buffer } from 'buffer';
import { LOCAL_STORAGE } from 'consts';
import { WALLET, WALLET_ID_LOCAL_STORAGE_KEY } from './consts';
import { WalletId } from './types';

function useWasmMetadata(source: RequestInfo | URL) {
  const alert = useAlert();
  const [data, setData] = useState<Buffer>();

  useEffect(() => {
    if (source) {
      fetch(source)
        .then((response) => response.arrayBuffer())
        .then((array) => Buffer.from(array))
        .then((buffer) => setData(buffer))
        .catch(({ message }: Error) => alert.error(`Fetch error: ${message}`));
    }
  }, [source, alert]);

  return { buffer: data };
}

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

function useWalletSync() {
  const { account, isAccountReady } = useAccount();
  const { address } = account || {};

  useEffect(() => {
    if (!isAccountReady) return;
    if (!account) return localStorage.removeItem(WALLET_ID_LOCAL_STORAGE_KEY);

    localStorage.setItem(WALLET_ID_LOCAL_STORAGE_KEY, account.meta.source);
  }, [isAccountReady, address, account]);
}

export { useWalletSync, useWallet, useWasmMetadata };
