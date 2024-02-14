import { useAccount, useApi, useReadFullState, useSendMessageHandler } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import metaTxt from 'assets/meta/galactic_express_meta.txt';
import { useProgramMetadata } from 'hooks';
import { LaunchState, PlayerInfo } from './types';
import { ADDRESS } from 'consts';
import { useAtomValue, useSetAtom } from 'jotai';
import { CURRENT_GAME_ATOM, PLAYER_INITIAL_STATUS_ATOM } from 'atoms';
import { useCallback, useEffect, useMemo } from 'react';

function useNewSessionMessage() {
  const meta = useProgramMetadata(metaTxt);

  return { meta: !!meta, message: useSendMessageHandler(ADDRESS.CONTRACT as HexString, meta, { isMaxGasLimit: true }) };
}

function useLaunchState() {
  const { account } = useAccount();
  const currentGame = useAtomValue(CURRENT_GAME_ATOM);
  const meta = useProgramMetadata(metaTxt);

  const payload = useMemo(
    () => ({ GetGame: { playerId: currentGame || account?.decodedAddress } }),
    [currentGame, account?.decodedAddress],
  );

  const { state } = useReadFullState<LaunchState>(ADDRESS.CONTRACT as HexString, meta, payload);
  console.log('STATE0');
  console.log(state?.Game);
  return state?.Game;
}

function useLaunchMessage() {
  const meta = useProgramMetadata(metaTxt);

  return { meta, message: useSendMessageHandler(ADDRESS.CONTRACT as HexString, meta, { isMaxGasLimit: true }) };
}

function usePlayerInfo() {
  const setPlayerInitialStatus = useSetAtom(PLAYER_INITIAL_STATUS_ATOM);
  const meta = useProgramMetadata(metaTxt);
  const { account } = useAccount();
  const { api } = useApi();

  const getPlayerStatus = useCallback(async () => {
    if (!account?.decodedAddress || !api) {
      return;
    }

    const payload = {
      GetPlayerInfo: {
        playerId: account.decodedAddress,
      },
    };

    const res = await api.programState.read(
      {
        programId: ADDRESS.CONTRACT,
        payload,
      },
      meta,
    );

    const state = (await res.toHuman()) as PlayerInfo;

    setPlayerInitialStatus(state.PlayerInfo);
  }, [account?.decodedAddress, api]);

  useEffect(() => {
    getPlayerStatus();
  }, [getPlayerStatus]);
}

export { useLaunchState, useLaunchMessage, useNewSessionMessage };
