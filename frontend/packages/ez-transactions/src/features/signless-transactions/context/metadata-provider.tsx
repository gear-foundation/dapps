import { HexString, ProgramMetadata } from '@gear-js/api';
import { ReactNode } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { Session, useCreateMetadataSession } from '../hooks';

import { usePair, useMetadataSession } from './hooks';
import { SignlessTransactionsContextWrapper } from './signless-transactions-context-wrapper';

type SignlessTransactionsMetadataProviderProps = {
  programId: HexString;
  metadataSource: string;
  children: ReactNode;
  isAutoSignlessEnabled?: boolean;
  /**
   * createSignatureType param is used when metadata.types.others.output has multiple types (e.g. tuple) to get the actual type for SignatureData
   */
  createSignatureType?: (metadata: ProgramMetadata, payloadToSig: Session) => `0x${string}`;
};

function SignlessTransactionsMetadataProvider({
  metadataSource,
  programId,
  children,
  isAutoSignlessEnabled = false,
  createSignatureType,
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
    isAutoSignlessEnabled,
  };

  return <SignlessTransactionsContextWrapper value={value}>{children}</SignlessTransactionsContextWrapper>;
}

export { SignlessTransactionsMetadataProvider };
export type { SignlessTransactionsMetadataProviderProps };
