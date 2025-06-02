import { CreateType } from '@gear-js/api';
import { useAccount, useApi, useBalance } from '@gear-js/react-hooks';
import { formatBalance } from '@polkadot/util';
import { useAtomValue, useSetAtom } from 'jotai';
import { useEffect } from 'react';

import { AVAILABLE_BALANCE, IS_AVAILABLE_BALANCE_READY } from './consts';
import { ISystemAccount } from './types';

export function useAccountAvailableBalance() {
  const isAvailableBalanceReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const availableBalance = useAtomValue(AVAILABLE_BALANCE);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);
  return { isAvailableBalanceReady, availableBalance, setAvailableBalance };
}

export function useAccountAvailableBalanceSync() {
  const { isAccountReady, account } = useAccount();
  const { api, isApiReady } = useApi();
  const { balance, isBalanceReady } = useBalance(account?.decodedAddress || '');

  const isReady = useAtomValue(IS_AVAILABLE_BALANCE_READY);
  const setIsReady = useSetAtom(IS_AVAILABLE_BALANCE_READY);
  const setAvailableBalance = useSetAtom(AVAILABLE_BALANCE);

  useEffect(() => {
    if (!api || !isApiReady || !isAccountReady) return;

    if (account && balance) {
      api.query.system.account(account.decodedAddress).then((res) => {
        const systemAccount = res.toJSON() as ISystemAccount;
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

          // `${value.slice(0, -2)}`
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
  }, [account, api, isAccountReady, isApiReady, isReady, setAvailableBalance, setIsReady, balance, isBalanceReady]);
}
