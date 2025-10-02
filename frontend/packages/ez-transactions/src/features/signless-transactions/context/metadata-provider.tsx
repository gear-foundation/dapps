import { HexString, ProgramMetadata } from '@gear-js/api';
import { ReactNode } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { DEFAULT_VOUCHER_ISSUE_AMOUNT, DEFAULT_VOUCHER_REISSUE_THRESHOLD } from '../consts';
import { Session, useCreateMetadataSession } from '../hooks';

import { SignlessTransactionsContext } from './context';
import { usePair, useMetadataSession } from './hooks';

type SignlessTransactionsMetadataProviderProps = {
  programId: HexString;
  metadataSource: string;
  children: ReactNode;
  /**
   * createSignatureType param is used when metadata.types.others.output has multiple types (e.g. tuple) to get the actual type for SignatureData
   */
  createSignatureType?: (metadata: ProgramMetadata, payloadToSig: Session) => `0x${string}`;
  voucherIssueAmount?: number;
  voucherReissueThreshold?: number;
};

function SignlessTransactionsMetadataProvider({
  metadataSource,
  programId,
  children,
  createSignatureType,
  voucherIssueAmount = DEFAULT_VOUCHER_ISSUE_AMOUNT,
  voucherReissueThreshold = DEFAULT_VOUCHER_REISSUE_THRESHOLD,
}: SignlessTransactionsMetadataProviderProps) {
  const metadata = useProgramMetadata(metadataSource);
  const { session, isSessionReady, isSessionActive } = useMetadataSession(programId, metadata);
  const { createSession, deleteSession } = useCreateMetadataSession(programId, metadata, createSignatureType);
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

export { SignlessTransactionsMetadataProvider };
export type { SignlessTransactionsMetadataProviderProps };
