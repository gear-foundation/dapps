import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';
import { GaslessContext } from './types';
import { DEFAULT_GASLESS_CONTEXT } from './consts';
import { useAccount, useAlert, useBalance, useBalanceFormat } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import { getVoucherId, getVoucherStatus } from './utils';
import { useLoading } from './hooks';

const GaslessTransactionsContext = createContext<GaslessContext>(DEFAULT_GASLESS_CONTEXT);
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

  const [accountAddress, setAccountAddress] = useState<string>();
  const [voucherId, setVoucherId] = useState<HexString>();
  const { balance } = useBalance(voucherId);

  const [isLoading, , withLoading] = useLoading();
  const [isAvailable, setIsAvailable] = useState(false);
  const [isEnabled, setIsEnabled] = useState(false);
  const isActive = Boolean(accountAddress && voucherId);

  const requestVoucher = async (_accountAddress: string) =>
    withLoading(
      getVoucherId(backendAddress, _accountAddress, programId).then((result) => {
        setAccountAddress(_accountAddress);
        setVoucherId(result);
      }),
    );

  useEffect(() => {
    withLoading(
      getVoucherStatus(backendAddress, programId)
        .then((result) => setIsAvailable(result))
        .catch(({ message }: Error) => alert.error(message)),
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!accountAddress || !balance) return;

    const isEnoughBalance = getChainBalanceValue(voucherLimit).isLessThan(balance.toString());
    if (isEnoughBalance) return;

    requestVoucher(accountAddress).catch(({ message }: Error) => alert.error(message));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [accountAddress, balance]);

  useEffect(() => {
    if (isEnabled) return;

    setAccountAddress(undefined);
    setVoucherId(undefined);
  }, [isEnabled]);

  useEffect(() => {
    setIsEnabled(false);
  }, [account]);

  const value = useMemo(
    () => ({ voucherId, isAvailable, isLoading, isEnabled, isActive, requestVoucher, setIsEnabled }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [voucherId, isAvailable, isLoading, isEnabled, isActive],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useGaslessTransactions = () => useContext(GaslessTransactionsContext);

export { DEFAULT_GASLESS_CONTEXT, GaslessTransactionsProvider, useGaslessTransactions };
export type { GaslessContext };