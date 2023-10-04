import { Buffer } from 'buffer';
import { useEffect, useState } from 'react';
import { useAtomValue, useSetAtom } from 'jotai';
import { CreateType } from '@gear-js/api';
import { formatBalance } from '@polkadot/util';
import { useAlert, useAccount, useApi } from '@gear-js/react-hooks';
import { LOCAL_STORAGE } from 'consts';
import { AVAILABLE_BALANCE, IS_AVAILABLE_BALANCE_READY, WALLET, WALLET_ID_LOCAL_STORAGE_KEY } from './consts';
import { SystemAccount, WalletId } from './types';

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

export function useAccountAvailableBalance() {
  const isAvailableBalanceReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const availableBalance = useAtomValue(AVAILABLE_BALANCE);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);
  return { isAvailableBalanceReady, availableBalance, setAvailableBalance };
}

export function useAccountAvailableBalanceSync() {
  const { isAccountReady, account } = useAccount();
  const { api, isApiReady } = useApi();

  const isReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const setIsReady = useSetAtom(IS_AVAILABLE_BALANCE_READY);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);

  useEffect(() => {
    if (!api || !isApiReady || !isAccountReady) return;

    if (account?.decodedAddress) {
      api.query.system.account(account.decodedAddress).then((res) => {
        const systemAccount = res.toJSON() as SystemAccount;
        const total = CreateType.create('u128', systemAccount.data.free).toString();
        const fee = CreateType.create('u128', systemAccount.data.feeFrozen).toString();

        const getBalance = (b: string) => () => {
          const [unit] = api.registry.chainTokens;
          const [decimals] = api.registry.chainDecimals;

          const value = formatBalance(b.toString(), {
            decimals,
            forceUnit: unit,
            withSiFull: false,
            withSi: false,
            withUnit: unit,
          });

          return { value, unit };
        };

        setAvailableBalance(getBalance(`${+total - +fee}`));
        if (!isReady) setIsReady(true);
      });
    } else setIsReady(true);
  }, [account, api, isAccountReady, isApiReady, isReady, setAvailableBalance, setIsReady]);
}

export { useWalletSync, useWallet, useWasmMetadata };
