import { useContext } from 'react';

import { DEFAULT_GASLESS_CONTEXT } from './consts';
import { GaslessTransactionsContext, GaslessTransactionsProvider } from './provider';
import { GaslessContext } from './types';

const useGaslessTransactions = () => useContext(GaslessTransactionsContext);

export { DEFAULT_GASLESS_CONTEXT, GaslessTransactionsProvider, useGaslessTransactions };
export type { GaslessContext };
