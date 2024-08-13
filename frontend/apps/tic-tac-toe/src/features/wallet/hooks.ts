import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { WALLET } from './consts';
import { IWalletId } from './types';

function useWallet() {
  const { wallets, account } = useAccount();

  const defaultWalletId = account?.meta.source as IWalletId | undefined;
  const [walletId, setWalletId] = useState(defaultWalletId);

  const wallet = walletId ? WALLET[walletId] : undefined;
  const walletAccounts = wallets && walletId ? wallets[walletId].accounts : undefined;

  useEffect(() => {
    setWalletId(defaultWalletId);
  }, [defaultWalletId]);

  const resetWalletId = () => setWalletId(undefined);

  return { wallet, walletId, walletAccounts, setWalletId, resetWalletId };
}

export { useWallet };
