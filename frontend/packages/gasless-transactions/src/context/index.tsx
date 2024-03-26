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

  const [voucherId, setVoucherId] = useState<HexString>();
  const { balance } = useBalance(voucherId);

  const [isLoading, , withLoading] = useLoading();
  const [isAvailable, setIsAvailable] = useState(false);
  const [isEnabled, setIsEnabled] = useState(false);

  // temporary? solution to demonstrate the ideal forkflow, where user:
  // checks the gasless -> starts game, or
  // checks the gasless -> creates signless session -> starts game.
  // cuz of gasless voucher balance check and update, signlessAccountAddress should be accessed somehow different.
  // good part about passing it as an argument is that signless pair is set after voucher request,
  // therefore it's requested voucher is accessible directly from the signless context via on chain call.
  const requestVoucher = async (signlessAccountAddress?: string) => {
    if (!account) throw new Error('Account is not found');
    const accountAddress = signlessAccountAddress || account.address;

    return withLoading(getVoucherId(backendAddress, accountAddress, programId).then((result) => setVoucherId(result)));
  };

  useEffect(() => {
    if (!account) {
      setIsAvailable(false);
      setIsEnabled(false);
      return;
    }

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

export { DEFAULT_GASLESS_CONTEXT, GaslessTransactionsProvider, useGaslessTransactions };
export type { GaslessContext };
