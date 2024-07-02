import { useMemo } from 'react';
import { useAccount, useReadFullState, useSendMessageWithGas } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import metaTxt from 'assets/meta/galactic_express_meta.txt';
import { useProgramMetadata } from 'hooks';
import { ADDRESS } from 'consts';
import { useAtomValue } from 'jotai';
import { CURRENT_GAME_ATOM } from 'atoms';
import { useDnsProgramIds } from '@dapps-frontend/hooks';
import { LaunchState } from './types';

function useLaunchState() {
  const { account } = useAccount();
  const { programId } = useDnsProgramIds();
  const currentGame = useAtomValue(CURRENT_GAME_ATOM);
  const meta = useProgramMetadata(metaTxt);

  const payload = useMemo(
    () => ({ GetGame: { playerId: currentGame || account?.decodedAddress } }),
    [currentGame, account?.decodedAddress],
  );

  const { state } = useReadFullState<LaunchState>(programId, meta, payload);

  return state?.Game;
}

function useLaunchMessage() {
  const { programId } = useDnsProgramIds();
  const meta = useProgramMetadata(metaTxt);

  return { meta: !!meta, message: useSendMessageWithGas(programId, meta, { isMaxGasLimit: true }) };
}

export { useLaunchState, useLaunchMessage };
