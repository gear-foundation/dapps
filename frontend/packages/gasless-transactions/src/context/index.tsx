import { ReactNode, createContext, useCallback, useContext, useEffect, useState } from 'react';
import { Value } from './types';
import { DEFAULT_VALUES } from './consts';
import { useAccount, useBalance, useBalanceFormat } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';

const GaslessTransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = GaslessTransactionsContext;

type Props = {
  programId: HexString;
  backendAddress: string;
  voucherLimit: number;
  children: ReactNode;
};

function GaslessTransactionsProvider({ backendAddress, programId, voucherLimit, children }: Props) {
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [voucherId, setVoucherId] = useState(undefined);
  const [isRequestingVoucher, setIsRequestingVoucher] = useState(false);
  const [isUpdatingVoucher, setIsUpdatingVoucher] = useState(false);

  const { balance } = useBalance(voucherId || account?.decodedAddress);

  const requestVoucher = async () => {
    try {
      const response = await fetch(`${backendAddress}api/voucher/request`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ account: account?.address, program: programId }),
      });

      const data = await response.json();

      if (data?.error) {
        console.log(`Voucher is not fetched - ${data.error}`);
      }

      return data.voucherId;
    } catch (error: any) {
      console.log('Error when fetching voucher');
      console.log(error);

      return undefined;
    }
  };

  const fetchVoucherId = async () => {
    try {
      setIsRequestingVoucher(true);
      const createdVoucherId = await requestVoucher();

      if (createdVoucherId) {
        setVoucherId(createdVoucherId);
      }

      setIsRequestingVoucher(false);
    } catch (error) {
      setIsRequestingVoucher(false);
    }
  };

  const updateBalance = useCallback(async () => {
    const formattedBalance = balance && getFormattedBalanceValue(balance.toString()).toFixed();
    const isBalanceLow = Number(formattedBalance) < voucherLimit;

    if (isBalanceLow && voucherId) {
      setIsUpdatingVoucher(true);

      try {
        const createdVoucherId = await requestVoucher();

        if (createdVoucherId) {
          setVoucherId(createdVoucherId);
        }

        setIsUpdatingVoucher(false);
      } catch (error) {
        console.log('error');
      }
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [balance]);

  useEffect(() => {
    if (account?.address) {
      fetchVoucherId();
    }
  }, [account?.address]);

  useEffect(() => {
    setVoucherId(undefined);
  }, [account?.address]);

  useEffect(() => {
    if (voucherId) {
      updateBalance();
    }
  }, [updateBalance, voucherId]);

  const value = {
    voucherId,
    isLoadingVoucher: isRequestingVoucher || isUpdatingVoucher,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useGaslessTransactions = () => useContext(GaslessTransactionsContext);

export { GaslessTransactionsProvider, useGaslessTransactions };
