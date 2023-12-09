import { HexString, ProgramMetadata } from '@gear-js/api';
import { useAccount, useReadFullState } from '@gear-js/react-hooks';
import { useMemo } from 'react';

import { State } from './types';

function useSession(programId: HexString, metadata: ProgramMetadata | undefined) {
  const { account } = useAccount();

  const payload = useMemo(() => ({ SessionForTheAccount: account?.decodedAddress }), [account]);
  const { state } = useReadFullState<State>(programId, metadata, payload);

  const session = state?.SessionForTheAccount;
  const isSessionReady = session !== undefined;

  return { session, isSessionReady };
}

export { useSession };
