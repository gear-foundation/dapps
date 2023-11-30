import {
  useAccount,
  useAlert,
  useApi,
  useBalance,
  useBalanceFormat,
  useVoucher,
  withoutCommas,
} from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import {
  AVAILABLE_BALANCE,
  IS_AVAILABLE_BALANCE_READY,
  VOUCHER_MIN_LIMIT,
  WALLET,
  WALLET_ID_LOCAL_STORAGE_KEY,
} from './consts';
import { SystemAccount, WalletId } from './types';
import { formatBalance, stringShorten } from '@polkadot/util';
import { CreateType } from '@gear-js/api';
import { useAtomValue, useSetAtom } from 'jotai';
import { ADDRESS } from '@/app/consts';
import { useFetchVoucher } from '@/app/hooks/useFetchVoucher';

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
    (localStorage.getItem(WALLET_ID_LOCAL_STORAGE_KEY) as WalletId | null) || undefined,
  );

  const wallet = walletId ? WALLET[walletId] : undefined;

  const getWalletAccounts = (id: WalletId) => accounts?.filter(({ meta }) => meta.source === id);
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

function useAccountAvailableBalance() {
  const isAvailableBalanceReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const availableBalance = useAtomValue(AVAILABLE_BALANCE);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);
  return { isAvailableBalanceReady, availableBalance, setAvailableBalance };
}

function useAccountAvailableBalanceSync() {
  const { isAccountReady, account } = useAccount();
  const { api, isApiReady } = useApi();
  const { balance } = useBalance(account?.decodedAddress);

  const isReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const setIsReady = useSetAtom(IS_AVAILABLE_BALANCE_READY);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);

  useEffect(() => {
    if (!api || !isApiReady || !isAccountReady) return;

    if (account && balance) {
      api.query.system.account(account.decodedAddress).then((res) => {
        const systemAccount = res.toJSON() as SystemAccount;

        const total = balance.toString();
        const fee = CreateType.create('u128', systemAccount.data.feeFrozen).toString();

        const getBalance = (b: string) => () => {
          const [unit] = api.registry.chainTokens;
          const [decimals] = api.registry.chainDecimals;

          const existentialDeposit = formatBalance(api.existentialDeposit, {
            decimals,
            forceUnit: unit,
            withSiFull: false,
            withSi: false,
            withUnit: unit,
            withZero: false,
          });

          const value = formatBalance(b.toString(), {
            decimals,
            forceUnit: unit,
            withSiFull: false,
            withSi: false,
            withUnit: unit,
          });

          return { value, unit, existentialDeposit };
        };

        setAvailableBalance(getBalance(`${+total - +fee}`));
        if (!isReady) {
          setIsReady(true);
        }
      });
    } else {
      setIsReady(true);
    }
  }, [account, api, isAccountReady, isApiReady, isReady, balance]);
}

function useCheckBalance() {
  const { api } = useApi();
  const { account } = useAccount();
  const { isVoucher } = useFetchVoucher(ADDRESS.GAME);
  const { availableBalance } = useAccountAvailableBalance();
  const { getChainBalanceValue } = useBalanceFormat();
  const { isVoucherExists, voucherBalance } = useVoucher(ADDRESS.GAME);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(getChainBalanceValue(Number(withoutCommas(availableBalance?.value || ''))).toFixed());
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()));
    const chainEDeposit = Number(
      getChainBalanceValue(Number(withoutCommas(availableBalance?.existentialDeposit || ''))).toFixed(),
    );

    const chainEDepositWithLimit = chainEDeposit + limit * valuePerGas;
    console.log(chainEDepositWithLimit);

    if (
      isVoucher && isVoucherExists && !!voucherBalance
        ? getFormattedBalanceValue(voucherBalance.toString()).toFixed() < VOUCHER_MIN_LIMIT
        : !chainBalance || chainBalance < chainEDepositWithLimit
    ) {
      alert.error(`Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`);

      if (onError) {
        onError();
      }

      return;
    }

    callback();
  };

  return { checkBalance };
}

export { useWalletSync, useWallet, useAccountAvailableBalance, useAccountAvailableBalanceSync, useCheckBalance };
