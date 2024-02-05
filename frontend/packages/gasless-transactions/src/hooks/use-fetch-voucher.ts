import { decodeAddress } from '@gear-js/api';
import { useBalanceFormat, useAccount, useIsVoucherExists, useVouchers, useBalance } from '@gear-js/react-hooks';

import { useSignlessTransactions } from '@dapps-frontend/signless-transactions';

import { useState, useEffect, useCallback } from 'react';
import { useAtom } from 'jotai';

import { IS_CREATING_VOUCHER_ATOM, IS_UPDATING_VOUCHER_ATOM } from '../atoms';
import { UseFetchVoucherProps } from '../types';

export function useFetchVoucher({ programId, backendAddress, voucherLimit = 18 }: UseFetchVoucherProps) {
  const { pair } = useSignlessTransactions();
  const { account } = useAccount();

  const accountAddress = pair ? decodeAddress(pair.address) : account?.decodedAddress;
  const { isVoucherExists } = useIsVoucherExists(programId, accountAddress);
  const { isEachVoucherReady, vouchers } = useVouchers(accountAddress, programId);
  const voucherKeys = isEachVoucherReady && vouchers ? Object.keys(vouchers) : [];
  const existingVoucherId = voucherKeys[0] as `0x${string}`;

  const { balance } = useBalance(vouchers && voucherKeys.length ? existingVoucherId : accountAddress);

  const { getFormattedBalanceValue } = useBalanceFormat();

  const [voucherId, setVoucherId] = useState<`0x${string}` | undefined>(undefined);
  const [isCreating, setIsCreating] = useAtom(IS_CREATING_VOUCHER_ATOM);
  const [isUpdating, setIsUpdating] = useAtom(IS_UPDATING_VOUCHER_ATOM);

  const createVoucher = async () => {
    try {
      const response = await fetch(backendAddress, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ account: accountAddress }),
      });

      if (response.status === 200) {
        const data = await response.json();
        return data;
      }
    } catch (error) {
      console.error('error creating voucher: ', error);
    }

    return false;
  };

  useEffect(() => {
    if (accountAddress && isVoucherExists !== undefined && backendAddress) {
      const fetchData = async () => {
        try {
          setIsCreating(true);
          const availableBack = await fetch(backendAddress);

          if (availableBack?.status === 200) {
            if (isVoucherExists) {
              setVoucherId(existingVoucherId);
            } else {
              const createdVoucherId = await createVoucher();

              if (createdVoucherId) {
                setVoucherId(createdVoucherId);
              }
            }
          }
          setIsCreating(false);
        } catch (error) {
          setIsCreating(false);
        }
      };

      fetchData();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [accountAddress, isVoucherExists, backendAddress]);

  const updateBalance = useCallback(async () => {
    const formattedBalance = balance && getFormattedBalanceValue(balance.toString()).toFixed();
    const isBalanceLow = Number(formattedBalance) < voucherLimit;

    if (isBalanceLow) {
      setIsUpdating(true);

      const createdVoucherId = await createVoucher();

      if (createdVoucherId) {
        setVoucherId(createdVoucherId);
      }

      setIsUpdating(false);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [balance]);

  useEffect(() => {
    setVoucherId(undefined);
  }, [accountAddress]);

  useEffect(() => {
    if (voucherId) {
      updateBalance();
    }
  }, [updateBalance, voucherId]);

  const isLoading = isCreating || isUpdating;

  return { voucherId, isLoading, updateBalance };
}
