import { useAlert } from '@gear-js/react-hooks';
import { useState } from 'react';
import { LOCAL_STORAGE } from 'consts';
import { WALLET } from './consts';
import { WalletId } from './types';

function useWallet() {
  const [walletId, setWalletId] = useState<WalletId | undefined>(localStorage[LOCAL_STORAGE.WALLET]);

  const resetWallet = () => setWalletId(undefined);

  const wallet = walletId ? WALLET[walletId] : undefined;

  return { wallet, walletId, switchWallet: setWalletId, resetWallet };
}

function useCopyToClipboard() {
  const alert = useAlert();

  const copy = (value: string) =>
    navigator.clipboard
      .writeText(value)
      .then(() => alert.success('Copied'))
      .catch(({ message }: Error) => alert.error(message));

  return copy;
}

export { useWallet, useCopyToClipboard };
