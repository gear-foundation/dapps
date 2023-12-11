import { useState, useEffect, useMemo, useCallback } from 'react';
import { useVoucher, useBalanceFormat, useAccount } from '@gear-js/react-hooks';
import { IS_CREATING_VOUCHER_ATOM, IS_UPDATING_VOUCHER_ATOM } from '../atoms';
import { useAtom } from 'jotai';
import { UseFetchVoucherProps } from '../types';

export function useFetchVoucher({ programId, backendAddress, voucherLimit }: UseFetchVoucherProps) {
  const { isVoucherExists, voucherBalance } = useVoucher(programId);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { account } = useAccount();
  const [voucher, setVoucher] = useState(false);
  const [isCreating, setIsCreating] = useAtom(IS_CREATING_VOUCHER_ATOM);
  const [isUpdating, setIsUpdating] = useAtom(IS_UPDATING_VOUCHER_ATOM);

  useEffect(() => {
    console.log('isVoucherExists:');
    console.log(isVoucherExists);
  }, [isVoucherExists]); // TODO remove before release

  useEffect(() => {
    console.log('Balance:');
    console.log(getFormattedBalanceValue(voucherBalance?.toString() || '').toFixed());
  }, [voucherBalance]); // TODO remove before release

  const createVoucher = async () => {
    try {
      const response = await fetch(backendAddress, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ account: account?.address }),
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
    if (account?.address && isVoucherExists !== undefined) {
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
  }, [account?.address, isVoucherExists]);

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
  }, [account?.address]);

  useEffect(() => {
    if (voucher) {
      updateBalance();
    }
  }, [updateBalance, voucher]);

  const isVoucher = useMemo(() => voucher, [voucher]);

  const isLoading = isCreating || isUpdating;

  return { isVoucher, isLoading, updateBalance };
}
