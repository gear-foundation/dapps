import { useEffect, useMemo } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import { NavigationType, useLocation, useNavigationType } from 'react-router-dom';
import { useProgramMetadata } from '@dapps-frontend/hooks';
import { useSignlessSendMessage } from '@dapps-frontend/ez-transactions';
import meta from './assets/meta/battleship.meta.txt';
import { IGameInstance } from './types';
import { gameAtom, gameModeAtom, isActiveGameAtom, pendingAtom } from './store';
import { ADDRESS } from './consts';
import { useReadState } from '@/app/hooks/api';
import { ROUTES } from '@/app/consts';

export function useGame() {
  const gameState = useAtomValue(gameAtom);
  const isActiveGame = useAtomValue(isActiveGameAtom);

  const setGameState = useSetAtom(gameAtom);
  const setActiveGame = useSetAtom(isActiveGameAtom);

  const resetGameState = () => {
    setGameState(undefined);
    setActiveGame(false);
  };

  return { gameState, isActiveGame, resetGameState, setGameState, setActiveGame };
}

function useGameState() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const programId = ADDRESS.GAME;
  const payload = useMemo(() => (decodedAddress ? { Game: decodedAddress } : undefined), [decodedAddress]);

  const { state: game, error } = useReadState<{ Game: IGameInstance | null }>({ programId, meta, payload });

  return { game, error };
}

export const useInitGame = () => {
  const { account } = useAccount();
  const { game, error } = useGameState();

  const { setGameState, resetGameState, setActiveGame } = useGame();

  useEffect(() => {
    if (!ADDRESS.GAME || !account?.decodedAddress) return;
    if (!game?.Game) return resetGameState();

    setGameState(game?.Game);
    setActiveGame(true);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress, game?.Game]);

  return {
    isGameReady: ADDRESS.GAME ? Boolean(game) : true,
    errorGame: error,
  };
};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);

  return useSignlessSendMessage(ADDRESS.GAME, metadata, { disableAlerts: true });
}

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}

export function useIsLocationGamePage() {
  const { pathname } = useLocation();

  return pathname === ROUTES.GAME;
}

export function useGameMode() {
  const [gameMode, setGameMode] = useAtom(gameModeAtom);

  const resetGameMode = () => {
    setGameMode(null);
  };

  return { gameMode, setGameMode, resetGameMode };
}
