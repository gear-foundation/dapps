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
  const [isAvailable, setIsAvailable] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isActive, setIsActive] = useState(false);
  const { balance } = useBalance(voucherId || account?.decodedAddress);

  const requestVoucher = async () => {
    try {
      const response = await fetch(`${backendAddress}gasless/voucher/request`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ account: account?.address, program: programId }),
      });

      const data = await response.json();
      setIsLoading(false);

      if (data?.error) {
        console.log(`Voucher is not fetched - ${data.error}`);
        setIsActive(false);

        return undefined;
      }

      if (!data.voucherId) {
        setIsActive(false);
        return undefined;
      }

      return data.voucherId;
    } catch (error: any) {
      console.log('Error when fetching voucher');
      console.log(error);
      setIsActive(false);
      setIsLoading(false);

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

  const checkProgramStatus = async () => {
    setIsLoading(true);

    try {
      const response = await fetch(`${backendAddress}gasless/voucher/${programId}/status`);

      const data = await response.json();
      setIsLoading(false);

      if (data.enabled) {
        setIsAvailable(true);
        return;
      }

      if (!data.enabled) {
        setIsAvailable(false);
        return;
      }

      console.log(`Backend is not available`);
      setIsAvailable(false);

      console.log(data);
    } catch (error: any) {
      console.log('Error when fetching voucher');
      console.log(error);
      setIsAvailable(false);
      setIsLoading(false);
    }
  };

  useEffect(() => {
    if (account?.decodedAddress) {
      setIsAvailable(false);
      setIsActive(false);
      setVoucherId(undefined);

      checkProgramStatus();
    }
  }, [account?.decodedAddress]);

  useEffect(() => {
    if (account?.decodedAddress && isAvailable && isActive && !voucherId) {
      setIsLoading(true);
      fetchVoucherId();
    }
  }, [isAvailable, isActive, voucherId, account?.decodedAddress]);

  useEffect(() => {
    if (voucherId) {
      updateBalance();
    }
  }, [updateBalance, voucherId]);

  const value = {
    voucherId: isAvailable && isActive ? voucherId : undefined,
    isLoadingVoucher: isRequestingVoucher || isUpdatingVoucher,
    isAvailable,
    isLoading,
    isActive,
    setIsActive,
    setIsLoading,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useGaslessTransactions = () => useContext(GaslessTransactionsContext);

export { GaslessTransactionsProvider, useGaslessTransactions };
