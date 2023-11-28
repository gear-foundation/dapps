import { useAccount, useReadFullState } from '@gear-js/react-hooks';
import { useMemo } from 'react';

import { ADDRESS } from '@/app/consts';
import { useProgramMetadata } from '@/app/hooks';
import metaTxt from '@/features/game/assets/meta/battleship.meta.txt';

import { State } from '../types';

function useSession() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const metadata = useProgramMetadata(metaTxt);

  const payload = useMemo(
    () => (decodedAddress ? { SessionForTheAccount: decodedAddress } : undefined),
    [decodedAddress],
  );

  const { state, isStateRead } = useReadFullState<State>(ADDRESS.GAME, metadata, payload);

  const session = state?.SessionForTheAccount;
  const isSessionReady = isStateRead;

  return { session, isSessionReady };
}

export { useSession };
