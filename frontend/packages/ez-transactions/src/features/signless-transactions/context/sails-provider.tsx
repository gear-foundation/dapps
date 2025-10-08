import { HexString } from '@gear-js/api';
import { ReactNode } from 'react';

import { useCreateSailsSession } from '../hooks';

import { usePair, useSailsSession } from './hooks';
import { SignlessTransactionsContextWrapper } from './signless-transactions-context-wrapper';
import { BaseProgram } from './types';

type SignlessTransactionsSailsProviderProps<TProgram extends BaseProgram> = {
  programId: HexString;
  children: ReactNode;
  program: TProgram;
  isAutoSignlessEnabled?: boolean;
  allowedActions?: string[];
};

function SignlessTransactionsSailsProvider<TProgram extends BaseProgram>({
  programId,
  children,
  program,
  isAutoSignlessEnabled = false,
  allowedActions,
}: SignlessTransactionsSailsProviderProps<TProgram>) {
  const { session, isSessionReady, isSessionActive } = useSailsSession(program);
  const { createSession, deleteSession } = useCreateSailsSession(programId, program);
  const pairData = usePair(programId, session);
  const value = {
    ...pairData,
    session,
    isSessionReady,
    createSession,
    deleteSession,
    isSessionActive,
    isAutoSignlessEnabled,
    allowedActions,
  };

  return <SignlessTransactionsContextWrapper value={value}>{children}</SignlessTransactionsContextWrapper>;
}

export { SignlessTransactionsSailsProvider };
export type { SignlessTransactionsSailsProviderProps };
