import { ReactNode, createContext, useContext } from 'react';

import { useGaslessTransactions } from '@dapps-frontend/gasless-transactions';
import { useSignlessTransactions } from '@dapps-frontend/signless-transactions';

import { DEFAULT_VALUES } from './consts';
import { Value } from './types';

const TransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = TransactionsContext;

type Props = {
  children: ReactNode;
};

function EzTransactionsProvider({ children }: Props) {
  const gasless = useGaslessTransactions();
  const signlessContext = useSignlessTransactions();

  const onSessionCreate = (signlessAccountAddress: string) => gasless.requestVoucher(signlessAccountAddress);

  const signless = {
    ...signlessContext,
    isActive: Boolean(signlessContext.pair), // TODO: move to signless context
    onSessionCreate,
  };

  return <Provider value={{ gasless, signless }}>{children}</Provider>;
}

const useEzTransactions = () => useContext(TransactionsContext);

export { EzTransactionsProvider, useEzTransactions };
