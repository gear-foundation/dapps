import { HexString } from '@gear-js/api';
import { ReactNode } from 'react';

import { DEFAULT_VOUCHER_REISSUE_THRESHOLD, DEFAULT_VOUCHER_ISSUE_AMOUNT } from '../consts';
import { useCreateSailsSession } from '../hooks';

import { SignlessTransactionsContext } from './context';
import { usePair, useSailsSession } from './hooks';
import { BaseProgram } from './types';

type SignlessTransactionsSailsProviderProps<TProgram extends BaseProgram> = {
  programId: HexString;
  children: ReactNode;
  program: TProgram;
  voucherIssueAmount?: number;
  voucherReissueThreshold?: number;
};

function SignlessTransactionsSailsProvider<TProgram extends BaseProgram>({
  programId,
  children,
  program,
  voucherIssueAmount = DEFAULT_VOUCHER_ISSUE_AMOUNT,
  voucherReissueThreshold = DEFAULT_VOUCHER_REISSUE_THRESHOLD,
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
    voucherIssueAmount,
    voucherReissueThreshold,
  };

  return <SignlessTransactionsContext.Provider value={value}>{children}</SignlessTransactionsContext.Provider>;
}

export { SignlessTransactionsSailsProvider };
export type { SignlessTransactionsSailsProviderProps };
