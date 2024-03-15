import { ReactNode, createContext, useContext } from 'react';

import { useGaslessTransactions } from '@/features/gasless-transactions';
import { useSignlessTransactions } from '@/features/signless-transactions';

import { DEFAULT_VALUES } from './consts';
import { Value } from './types';

const TransactionsContext = createContext<Value>(DEFAULT_VALUES);
const { Provider } = TransactionsContext;

type Props = {
  children: ReactNode;
};

function TransactionsProvider({ children }: Props) {
  const gasless = useGaslessTransactions();
  const signless = useSignlessTransactions();

  return <Provider value={{ gasless, signless }}>{children}</Provider>;
}

const useTransactions = () => useContext(TransactionsContext);

export { TransactionsProvider, useTransactions };
