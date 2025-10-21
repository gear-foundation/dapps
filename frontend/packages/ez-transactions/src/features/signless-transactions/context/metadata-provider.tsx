import { HexString, ProgramMetadata } from '@gear-js/api';
import { ReactNode } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { DEFAULT_VOUCHER_ISSUE_AMOUNT, DEFAULT_VOUCHER_REISSUE_THRESHOLD } from '../consts';
import { Session, useCreateMetadataSession } from '../hooks';

import { usePair, useMetadataSession } from './hooks';
import { SignlessTransactionsModalProvider } from './signless-transactions-modal-provider';

type SignlessTransactionsMetadataProviderProps = {
  programId: HexString;
  metadataSource: string;
  children: ReactNode;
  isAutoSignlessEnabled?: boolean;
  allowedActions?: string[];
  /**
   * createSignatureType param is used when metadata.types.others.output has multiple types (e.g. tuple) to get the actual type for SignatureData
   */
  createSignatureType?: (metadata: ProgramMetadata, payloadToSig: Session) => `0x${string}`;
  voucherIssueAmount?: number;
  voucherReissueThreshold?: number;
  allowIncreaseVoucherValue?: boolean;
};

function SignlessTransactionsMetadataProvider({
  metadataSource,
  programId,
  children,
  isAutoSignlessEnabled = false,
  allowedActions,
  createSignatureType,
  voucherIssueAmount = DEFAULT_VOUCHER_ISSUE_AMOUNT,
  voucherReissueThreshold = DEFAULT_VOUCHER_REISSUE_THRESHOLD,
  allowIncreaseVoucherValue = false,
}: SignlessTransactionsMetadataProviderProps) {
  const metadata = useProgramMetadata(metadataSource);
  const { session, isSessionReady, isSessionActive } = useMetadataSession(programId, metadata);
  const { createSession, deleteSession, updateVoucherBalance } = useCreateMetadataSession(
    programId,
    metadata,
    createSignatureType,
  );
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
  };

  return <SignlessTransactionsModalProvider value={value}>{children}</SignlessTransactionsModalProvider>;
}

export { SignlessTransactionsMetadataProvider };
export type { SignlessTransactionsMetadataProviderProps };
