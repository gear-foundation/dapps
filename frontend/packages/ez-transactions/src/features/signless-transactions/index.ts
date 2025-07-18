import { SignlessTransactions, SignlessActive, EnableSignlessSession, CreateSessionModal } from './components';
import {
  SignlessTransactionsProvider,
  useSignlessTransactions,
  DEFAULT_SIGNLESS_CONTEXT,
  SignlessContext,
  SignlessTransactionsMetadataProviderProps,
  SignlessTransactionsSailsProviderProps,
  SignlessTransactionsContext,
  usePair,
} from './context';
import {
  useSignlessSendMessage,
  useSignlessSendMessageHandler,
  SendSignlessMessageOptions,
  useRandomPairOr,
  useCreateBaseSession,
  useCreateSailsSession,
} from './hooks';
import { signHex, getUnlockedPair } from './utils';

export {
  SignlessTransactions,
  SignlessActive,
  CreateSessionModal,
  SignlessTransactionsContext,
  SignlessTransactionsProvider,
  EnableSignlessSession,
  useSignlessSendMessage,
  useSignlessSendMessageHandler,
  useSignlessTransactions,
  useRandomPairOr,
  DEFAULT_SIGNLESS_CONTEXT,
  signHex,
  getUnlockedPair,
  useCreateBaseSession,
  useCreateSailsSession,
  usePair,
};

export type {
  SendSignlessMessageOptions,
  SignlessContext,
  SignlessTransactionsMetadataProviderProps,
  SignlessTransactionsSailsProviderProps,
};
