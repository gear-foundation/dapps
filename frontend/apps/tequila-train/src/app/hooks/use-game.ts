import { useApp, useGame } from 'app/context';
import { useEffect, useMemo } from 'react';
import { useAccount, useSendMessageHandler } from '@gear-js/react-hooks';
import { ENV } from 'app/consts';
import meta from 'assets/meta/tequila_train.meta.txt';

import { IState } from '../types/game';
import { useProgramMetadata, useReadState } from './use-metadata';

const programIdGame = ENV.game;

function useGameState() {
  const { account } = useAccount();

  const payloadGame = useMemo(
    () =>
      account?.decodedAddress
        ? account.decodedAddress
        : undefined,
    [account?.decodedAddress],
  );

  const { state, error } = useReadState<IState>({
    programId: programIdGame,
    meta,
    payload: payloadGame,
  });
  
  return { state, error };
}

export const useInitGame = () => {
  const { setIsAllowed, setOpenWinnerPopup } = useApp();
  const { account } = useAccount();
  const { state } = useGameState();
  const { setGame, setPlayers } = useGame();

  useEffect(() => {
    setGame(state);
    if (state && account && state.isStarted && state?.gameState) {
      setPlayers(state.players);

      setIsAllowed(account.decodedAddress === state.players[+state.gameState?.currentPlayer][0]);
      if (state.gameState?.state?.Winner) {
        setOpenWinnerPopup(true);
      }
    } else {
      setPlayers([]);
      setIsAllowed(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, account?.address, state?.gameState]);
};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSendMessageHandler(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  });
}
