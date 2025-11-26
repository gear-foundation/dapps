import { HexString } from '@gear-js/api';
import { ReactNode } from 'react';

import { DEFAULT_VOUCHER_REISSUE_THRESHOLD, DEFAULT_VOUCHER_ISSUE_AMOUNT } from '../consts';
import { useCreateSailsSession } from '../hooks';

import { usePair, useSailsSession } from './hooks';
import { SignlessTransactionsModalProvider } from './signless-transactions-modal-provider';
import { BaseProgram } from './types';

type SignlessTransactionsSailsProviderProps<TProgram extends BaseProgram> = {
  programId: HexString;
  children: ReactNode;
  program: TProgram;
  voucherIssueAmount?: number;
  voucherReissueThreshold?: number;
  isAutoSignlessEnabled?: boolean;
  allowedActions?: string[];
  allowIncreaseVoucherValue?: boolean;
  defaultDurationMinutes?: string;
};

function SignlessTransactionsSailsProvider<TProgram extends BaseProgram>({
  programId,
  children,
  program,
  voucherIssueAmount = DEFAULT_VOUCHER_ISSUE_AMOUNT,
  voucherReissueThreshold = DEFAULT_VOUCHER_REISSUE_THRESHOLD,
  isAutoSignlessEnabled = false,
  allowedActions,
  allowIncreaseVoucherValue = false,
  defaultDurationMinutes,
}: SignlessTransactionsSailsProviderProps<TProgram>) {
  const { session, isSessionReady, isSessionActive } = useSailsSession(program);
  const { createSession, deleteSession, updateVoucherBalance } = useCreateSailsSession(programId, program);
  const pairData = usePair(programId, session);
  const value = {
    ...pairData,
    session,
    isSessionReady,
    createSession,
    deleteSession,
    updateVoucherBalance,
    isSessionActive,
    voucherIssueAmount,
    voucherReissueThreshold,
    isAutoSignlessEnabled,
    allowedActions,
    allowIncreaseVoucherValue,
    defaultDurationMinutes,
  };

  return <SignlessTransactionsModalProvider value={value}>{children}</SignlessTransactionsModalProvider>;
}

export { SignlessTransactionsSailsProvider };
export type { SignlessTransactionsSailsProviderProps };
