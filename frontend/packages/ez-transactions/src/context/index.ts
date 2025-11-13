import { useContext } from 'react';

import { EzTransactionsProvider, TransactionsContext } from './provider';

const useEzTransactions = () => useContext(TransactionsContext);

export { EzTransactionsProvider, useEzTransactions };
