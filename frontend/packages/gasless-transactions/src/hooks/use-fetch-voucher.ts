import { useState, useEffect, useMemo, useCallback } from 'react';
import { useVoucher, useBalanceFormat } from '@gear-js/react-hooks';
import { IS_CREATING_VOUCHER_ATOM, IS_UPDATING_VOUCHER_ATOM } from '../atoms';
import { useAtom } from 'jotai';
import { UseFetchVoucherProps } from '../types';

export function useFetchVoucher({ accountAddress, programId, backendAddress, voucherLimit }: UseFetchVoucherProps) {
  const { isVoucherExists, voucherBalance } = useVoucher(programId);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [voucher, setVoucher] = useState(false);
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
        return true;
      }
    } catch (error) {
      console.log('error: ', error);
    }

    return false;
  };

  useEffect(() => {
    if (accountAddress && isVoucherExists !== undefined) {
      const fetchData = async () => {
        try {
          setIsCreating(true);
          const availableBack = await fetch(backendAddress);

          if (availableBack?.status === 200) {
            if (isVoucherExists) {
              console.log('EXISTS');
              setVoucher(true);
            } else {
              const createdVoucher = await createVoucher();
              if (createdVoucher) {
                console.log('CREATED');
                setVoucher(true);
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
  }, [accountAddress, isVoucherExists]);

  const updateBalance = useCallback(async () => {
    const formattedBalance = voucherBalance && getFormattedBalanceValue(voucherBalance.toString()).toFixed();
    const isBalanceLow = formattedBalance < (voucherLimit || 18);

    if (isBalanceLow) {
      setIsUpdating(true);

      const createdVoucher = await createVoucher();

      if (createdVoucher) {
        setVoucher(true);
      }

      setIsUpdating(false);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [voucherBalance]);

  useEffect(() => {
    setVoucher(false);
  }, [accountAddress]);

  useEffect(() => {
    if (voucher) {
      updateBalance();
    }
  }, [updateBalance, voucher]);

  const isVoucher = useMemo(() => voucher, [voucher]);

  const isLoading = isCreating || isUpdating;

  return { isVoucher, isLoading, updateBalance };
}
