import { HexString } from '@gear-js/api';
import { useAccount, useAlert, useBalance, useBalanceFormat } from '@gear-js/react-hooks';
import { ReactNode, createContext, useContext, useEffect, useMemo, useState } from 'react';

import { DEFAULT_GASLESS_CONTEXT } from './consts';
import { useLoading } from './hooks';
import { GaslessContext, VoucherStatus } from './types';
import { getVoucherId, getVoucherStatus } from './utils';

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
  const [voucherId, setVoucherId] = useState<HexString | undefined>();
  const { balance } = useBalance(voucherId);

  const [isLoading, , withLoading] = useLoading();
  const [isEnabled, setIsEnabled] = useState(false);
  const isActive = Boolean(accountAddress && voucherId);
  const [voucherStatus, setVoucherStatus] = useState<VoucherStatus | null>(null);

  const expireTimestamp = useMemo(
    () => (voucherId && voucherStatus ? Date.now() + voucherStatus.duration * 1000 : null),
    [voucherId, voucherStatus],
  );

  const requestVoucher = async (_accountAddress: string, isSaveContext = true) =>
    withLoading(
      getVoucherId(backendAddress, _accountAddress, programId).then((result) => {
        if (isSaveContext) {
          setAccountAddress(_accountAddress);
          setVoucherId(result);
        }

        return result;
      }),
    );

  useEffect(() => {
    withLoading(
      getVoucherStatus(backendAddress, programId)
        .then((result) => {
          setVoucherStatus(result);
        })
        .catch(({ message }: Error) => alert.error(message)),
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [programId]);

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
    () => ({
      voucherId,
      isLoading,
      isEnabled,
      isActive,
      voucherStatus,
      expireTimestamp,
      requestVoucher,
      setIsEnabled,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [voucherId, isLoading, isEnabled, isActive, voucherStatus?.id, expireTimestamp],
  );

  return <Provider value={value}>{children}</Provider>;
}

const useGaslessTransactions = () => useContext(GaslessTransactionsContext);

export { DEFAULT_GASLESS_CONTEXT, GaslessTransactionsProvider, useGaslessTransactions };
export type { GaslessContext };
