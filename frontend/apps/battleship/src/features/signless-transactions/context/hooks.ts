import { useAccount, useReadFullState } from '@gear-js/react-hooks';
import { useMemo } from 'react';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { ADDRESS } from '@/app/consts';
import metaTxt from '@/features/game/assets/meta/battleship.meta.txt';

import { State } from './types';

function useSession() {
  const { account } = useAccount();
  const metadata = useProgramMetadata(metaTxt);

  const payload = useMemo(() => ({ SessionForTheAccount: account?.decodedAddress }), [account]);
  const { state } = useReadFullState<State>(ADDRESS.GAME, metadata, payload);

  const session = state?.SessionForTheAccount;
  const isSessionReady = session !== undefined;

  return { session, isSessionReady };
}

export { useSession };
