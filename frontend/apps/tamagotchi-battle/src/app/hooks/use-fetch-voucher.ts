import { useState, useEffect, useMemo, useCallback } from 'react';
import { useVoucher, useBalanceFormat } from '@gear-js/react-hooks';
import { ENV, IS_CREATING_VOUCHER_ATOM, IS_UPDATING_VOUCHER_ATOM, VOUCHER_MIN_LIMIT } from '../consts';
import { BATTLE_ADDRESS } from 'features/battle/consts';
import { useAtom } from 'jotai';

export function useFetchVoucher(account: string | undefined) {
  const { isVoucherExists, voucherBalance } = useVoucher(BATTLE_ADDRESS);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [voucher, setVoucher] = useState(false);
  const [isCreating, setIsCreating] = useAtom(IS_CREATING_VOUCHER_ATOM);
  const [isUpdating, setIsUpdating] = useAtom(IS_UPDATING_VOUCHER_ATOM);

  const createVoucher = async () => {
    try {
      const response = await fetch(ENV.BACK, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ account }),
      });

      if (response.status === 200) {
        return true;
      }
    } catch (error) {
      console.log('error: ', error);
    }

    return false;
  };

  console.log(isVoucherExists);
  console.log(getFormattedBalanceValue(voucherBalance?.toString() || '').toFixed());

  useEffect(() => {
    if (account && isVoucherExists !== undefined) {
      const fetchData = async () => {
        try {
          setIsCreating(true);
          const availableBack = await fetch(ENV.BACK);

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
  }, [account, isVoucherExists]);

  const updateBalance = useCallback(async () => {
    const formattedBalance = voucherBalance && getFormattedBalanceValue(voucherBalance.toString()).toFixed();
    const isBalanceLow = formattedBalance < VOUCHER_MIN_LIMIT;

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
  }, [account]);

  useEffect(() => {
    if (voucher) {
      updateBalance();
    }
  }, [updateBalance, voucher]);

  const isVoucher = useMemo(() => voucher, [voucher]);

  const isLoading = isCreating || isUpdating;

  return { isVoucher, isLoading, updateBalance };
}
