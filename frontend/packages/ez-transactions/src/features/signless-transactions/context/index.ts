import { useContext } from 'react';

import { DEFAULT_SIGNLESS_CONTEXT } from './consts';
import { SignlessTransactionsContext } from './context';
import { SignlessTransactionsMetadataProviderProps } from './metadata-provider';
import { SignlessTransactionsProvider } from './provider';
import { SignlessTransactionsSailsProviderProps } from './sails-provider';
import { SignlessContext, SignlessSessionModalConfig } from './types';

const useSignlessTransactions = () => useContext(SignlessTransactionsContext);

export { SignlessTransactionsProvider, useSignlessTransactions, DEFAULT_SIGNLESS_CONTEXT };
export type {
  SignlessContext,
  SignlessSessionModalConfig,
  SignlessTransactionsMetadataProviderProps,
  SignlessTransactionsSailsProviderProps,
};
