import { useState, useEffect, useMemo, useCallback } from 'react';
import { useVoucher, useBalanceFormat } from '@gear-js/react-hooks'
import { ADDRESS } from '../consts';

export function useFetchVoucher(account: string | undefined) {
    const { isVoucherExists, voucherBalance } = useVoucher(ADDRESS.GAME);
    const { getFormattedBalanceValue } = useBalanceFormat()

    const [voucher, setVoucher] = useState(false);
    const [isLoading, setIsLoading] = useState(false);


    const fetchVoucher = async () => {
        try {
            const response = await fetch(ADDRESS.BACK, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ account })
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
        if (account && (isVoucherExists !== undefined)) {
            const fetchData = async () => {
                setIsLoading(true);
                const availableBack = await fetch(ADDRESS.BACK)
                if (availableBack?.status === 200) {
                    if (isVoucherExists) {
                        setVoucher(true);
                    } else {

                    }
                }
                setIsLoading(false);
            };

            fetchData();
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [account, isVoucherExists]);

    const updateBalance = useCallback(async () => {
        const formattedBalance = voucherBalance && getFormattedBalanceValue(voucherBalance).toFixed();
        const isBalanceLow = formattedBalance < 3;

        if (isBalanceLow) {
            const createdVoucher = await fetchVoucher();
            if (createdVoucher) {

                setVoucher(true);
            }
        }

        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [voucherBalance]);

    const memoizedVoucher = useMemo(() => voucher, [voucher]);

    return { memoizedVoucher, isLoading, updateBalance };
}