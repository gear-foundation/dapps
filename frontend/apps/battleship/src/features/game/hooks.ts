import { useEffect, useMemo } from 'react';
import { useAccount, useSendMessageHandler } from '@gear-js/react-hooks';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import meta from './assets/meta/battleship.meta.txt';
import { IGameInstance } from './types';
import { gameAtom, isActiveGameAtom, pendingAtom } from './store';
import { ADDRESS } from './consts';
import { useProgramMetadata } from '@/app/hooks';
import { useReadState } from '@/app/hooks/api';

const programIdGame = ADDRESS.GAME;

export function useGame() {
  const setGameState = useSetAtom(gameAtom);
  const gameState = useAtomValue(gameAtom);
  const isActiveGame = useAtomValue(isActiveGameAtom);
  const setActiveGame = useSetAtom(isActiveGameAtom);

  const resetGameState = () => {
    setGameState(undefined);
    setActiveGame(false);
  };

  return {
    resetGameState,
    setGameState,
    gameState,
    isActiveGame,
    setActiveGame,
  };
}

function useGameState() {
  const { account } = useAccount();

  const payloadGame = useMemo(
    () =>
      account?.decodedAddress
        ? {
            Game: account.decodedAddress,
          }
        : undefined,
    [account?.decodedAddress],
  );

  const { state: game, error } = useReadState<{ Game: IGameInstance | null }>({
    programId: programIdGame,
    meta,
    payload: payloadGame,
  });

  return { game, error };
}

export const useInitGame = () => {
  const { account } = useAccount();
  const { game, error } = useGameState();

  const { setGameState, resetGameState, setActiveGame } = useGame();

  useEffect(() => {
    if (programIdGame && account?.decodedAddress) {
      if (game?.Game) {
        setGameState(game?.Game);
        setActiveGame(true);
      } else {
        resetGameState();
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress, game?.Game]);

  return {
    isGameReady: programIdGame ? Boolean(game) : true,
    errorGame: error,
  };
};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSendMessageHandler(programIdGame, metadata, {
    disableAlerts: true,
  });
}

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);
  return { pending, setPending };
}
