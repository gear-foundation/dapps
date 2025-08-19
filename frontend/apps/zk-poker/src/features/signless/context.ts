import { HexString } from '@gear-js/api';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';
import { PrepareEzTransactionParamsResult } from 'gear-ez-transactions';
import { createContext } from 'react';

interface TransactionWithSessionOptions {
  onSuccess?: () => void;
  onError?: (error: Error) => void;
  onFinally?: () => void;
}

type Transaction = TransactionReturn<(...args: unknown[]) => GenericTransactionReturn<null>>;

type PrepareTransactionAsyncResult = Promise<{
  transaction: Transaction;
  awaited: {
    fee: bigint;
  };
}>;
interface AutoSignlessContextType {
  executeWithSessionModal: (
    getTransaction: (params?: Partial<PrepareEzTransactionParamsResult>) => PrepareTransactionAsyncResult,
    sessionForAccount: HexString | null,
    options?: TransactionWithSessionOptions,
  ) => Promise<void>;
  closeModal: () => void;
}

const AutoSignlessContext = createContext<AutoSignlessContextType | undefined>(undefined);

export {
  AutoSignlessContext,
  type AutoSignlessContextType,
  type TransactionWithSessionOptions,
  type PrepareTransactionAsyncResult,
  type Transaction,
};
