import { useAtomValue, useSetAtom } from 'jotai';
import { useEffect } from 'react';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { CreateType } from '@gear-js/api';
import { formatBalance } from '@polkadot/util';
import { ISystemAccount } from './types';
import { AVAILABLE_BALANCE, IS_AVAILABLE_BALANCE_LOADING, IS_AVAILABLE_BALANCE_READY } from './store';

export function useAccountAvailableBalance() {
  const isAvailableBalanceReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const availableBalance = useAtomValue(AVAILABLE_BALANCE);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);
  const isAvailableBalanceLoading = useAtomValue(IS_AVAILABLE_BALANCE_LOADING);
  return {
    isAvailableBalanceReady,
    availableBalance,
    isAvailableBalanceLoading,
    setAvailableBalance,
  };
}

export function useAccountAvailableBalanceSync() {
  const { isAccountReady, account } = useAccount();
  const { api, isApiReady } = useApi();

  const isReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const setIsReady = useSetAtom(IS_AVAILABLE_BALANCE_READY);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);
  const setIsLoading = useSetAtom(IS_AVAILABLE_BALANCE_LOADING);

  useEffect(() => {
    if (!api || !isApiReady || !isAccountReady) return;

    if (account?.decodedAddress) {
      setIsLoading(true);

      api.query.system.account(account.decodedAddress).then((res) => {
        const systemAccount = res.toJSON() as ISystemAccount;
        const total = CreateType.create('u128', systemAccount.data.free).toString();
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

          // `${value.slice(0, -2)}`
          return { value, unit, existentialDeposit };
        };

        setAvailableBalance(getBalance(`${+total - +fee}`));
        setIsLoading(false);
        if (!isReady) setIsReady(true);
      });
    } else setIsReady(true);
  }, [
    account?.decodedAddress,
    api,
    isAccountReady,
    isApiReady,
    isReady,
    setAvailableBalance,
    setIsReady,
    setIsLoading,
  ]);
}
