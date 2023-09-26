import { useState, useEffect, useMemo } from 'react';
import { ADDRESS } from '../consts';

export function useFetchVoucher(account: string | undefined) {
    const [voucher, setVoucher] = useState(false);

    useEffect(() => {
        if (account) {
            const fetchData = async () => {
                try {
                    const response = await fetch(ADDRESS.BACK, {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify({ account })
                    })

                    if (response.status === 200) {
                        setVoucher(true);
                    }
                } catch (error) {
                    console.log('error: ', error)
                }
            };

            fetchData();
        }
    }, [account]);

    const memoizedVoucher = useMemo(() => voucher, [voucher]);
    
    return memoizedVoucher;
}