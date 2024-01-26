import {
  useAccount,
  useAlert,
  useApi,
  useBalance,
  useBalanceFormat,
  useHandleCalculateGas as useCalculateGasNative,
  useVoucherBalance,
  withoutCommas,
} from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAtomValue, useSetAtom } from 'jotai';
import { useSignlessTransactions } from '@dapps-frontend/signless-transactions';
import { useProgramState, useSubscriptionsMessage } from './api';
import { AnyJson, AnyNumber } from '@polkadot/types/types';
import { CreateType, HexString, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { SystemAccount } from 'types';
import { formatBalance, stringShorten } from '@polkadot/util';
import { AVAILABLE_BALANCE, IS_AVAILABLE_BALANCE_READY } from 'atoms';
import { ADDRESS, VOUCHER_MIN_LIMIT } from 'consts';

function useSubscription() {
  const navigate = useNavigate();

  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { subscriptionsState, isSubscriptionsStateRead } = useProgramState();

  const subscription = subscriptionsState && decodedAddress ? subscriptionsState[decodedAddress] : undefined;

  useEffect(() => {
    if (isSubscriptionsStateRead && !subscription) {
      navigate('/subscription');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isSubscriptionsStateRead, subscription, account]);

  return isSubscriptionsStateRead;
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

function useCheckBalance(isVoucher?: boolean) {
  const { api } = useApi();

  const { account } = useAccount();
  const { pair } = useSignlessTransactions();
  const accountAddress = pair ? decodeAddress(pair.address) : account?.decodedAddress;
  const { voucherBalance } = useVoucherBalance(ADDRESS.CONTRACT, accountAddress);

  const { availableBalance } = useAccountAvailableBalance();
  const { getChainBalanceValue } = useBalanceFormat();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(getChainBalanceValue(Number(withoutCommas(availableBalance?.value || ''))).toFixed());
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()));
    const chainEDeposit = Number(
      getChainBalanceValue(Number(withoutCommas(availableBalance?.existentialDeposit || ''))).toFixed(),
    );

    const chainEDepositWithLimit = chainEDeposit + limit * valuePerGas;

    if (
      isVoucher && !!voucherBalance
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

const useHandleCalculateGas = (address: HexString, meta: ProgramMetadata | undefined) => {
  const { availableBalance } = useAccountAvailableBalance();
  const calculateGasNative = useCalculateGasNative(address, meta);

  const alert = useAlert();

  return (initPayload: AnyJson, value?: AnyNumber | undefined) => {
    const balance = Number(withoutCommas(availableBalance?.value || ''));
    const existentialDeposit = Number(withoutCommas(availableBalance?.existentialDeposit || ''));

    if (!balance || balance < existentialDeposit) {
      alert.error(`Low balance when calculating gas`);
    }

    return calculateGasNative(initPayload, value);
  };
};

export {
  useSubscriptionsMessage,
  useSubscription,
  useHandleCalculateGas,
  useAccountAvailableBalance,
  useAccountAvailableBalanceSync,
  useCheckBalance,
};
