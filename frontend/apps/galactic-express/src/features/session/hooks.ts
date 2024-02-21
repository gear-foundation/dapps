import { useMemo } from 'react';
import { useAccount, useReadFullState, useSendMessageHandler } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import metaTxt from 'assets/meta/galactic_express_meta.txt';
import { useProgramMetadata } from 'hooks';
import { ADDRESS } from 'consts';
import { useAtomValue } from 'jotai';
import { CURRENT_GAME_ATOM } from 'atoms';
import { LaunchState } from './types';

function useLaunchState() {
  const { account } = useAccount();
  const currentGame = useAtomValue(CURRENT_GAME_ATOM);
  const meta = useProgramMetadata(metaTxt);

  const payload = useMemo(
    () => ({ GetGame: { playerId: currentGame || account?.decodedAddress } }),
    [currentGame, account?.decodedAddress],
  );

  const { state } = useReadFullState<LaunchState>(ADDRESS.CONTRACT as HexString, meta, payload);

  return state?.Game;
}

function useLaunchMessage() {
  const meta = useProgramMetadata(metaTxt);

  return { meta: !!meta, message: useSendMessageHandler(ADDRESS.CONTRACT as HexString, meta, { isMaxGasLimit: true }) };
}

export { useLaunchState, useLaunchMessage };
