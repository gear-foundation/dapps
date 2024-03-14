import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';
import { Value } from './types';
import { DEFAULT_VALUES } from './consts';
import { useAccount, useAlert, useBalance, useBalanceFormat } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import { getVoucherId, getVoucherStatus } from './utils';
import { useLoading } from './hooks';

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
  const { getChainBalanceValue } = useBalanceFormat();
  const alert = useAlert();

  const [voucherId, setVoucherId] = useState<HexString>();
  const { balance } = useBalance(voucherId);

  const [isLoading, , withLoading] = useLoading();
  const [isAvailable, setIsAvailable] = useState(false);
  const [isEnabled, setIsEnabled] = useState(false);

  const requestVoucher = async () => {
    if (!account) throw new Error('Account is not found');

    return withLoading(getVoucherId(backendAddress, account.address, programId).then((result) => setVoucherId(result)));
  };

  useEffect(() => {
    if (!account) return setIsAvailable(false);

    withLoading(
      getVoucherStatus(backendAddress, programId)
        .then((result) => setIsAvailable(result))
        .catch(({ message }: Error) => alert.error(message)),
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account]);

  useEffect(() => {
    if (!balance) return;

    const isEnoughBalance = getChainBalanceValue(voucherLimit).isLessThan(balance.toString());
    if (isEnoughBalance) return;

    requestVoucher().catch(({ message }: Error) => alert.error(message));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [balance]);

  useEffect(() => {
    if (isEnabled) return;

    setVoucherId(undefined);
  }, [isEnabled]);

  const value = useMemo(
    () => ({ voucherId, isAvailable, isLoading, isEnabled, requestVoucher, setIsEnabled }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [voucherId, isAvailable, isLoading, isEnabled, account],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useGaslessTransactions = () => useContext(GaslessTransactionsContext);

export { GaslessTransactionsProvider, useGaslessTransactions };
