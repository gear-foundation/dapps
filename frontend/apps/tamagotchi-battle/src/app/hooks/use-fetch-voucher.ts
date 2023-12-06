import { useState, useEffect, useMemo, useCallback } from 'react';
import { useVoucher, useBalanceFormat } from '@gear-js/react-hooks';
import { ENV, VOUCHER_MIN_LIMIT } from '../consts';
import { BATTLE_ADDRESS } from 'features/battle/consts';

export function useFetchVoucher(account: string | undefined) {
  const { isVoucherExists, voucherBalance } = useVoucher(BATTLE_ADDRESS);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [voucher, setVoucher] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

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
          setIsLoading(true);
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
          setIsLoading(false);
        } catch (error) {
          setIsLoading(false);
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
      const createdVoucher = await createVoucher();

      if (createdVoucher) {
        setVoucher(true);
      }
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [voucherBalance]);

  const isVoucher = useMemo(() => voucher, [voucher]);

  return { isVoucher, isLoading, updateBalance };
}
