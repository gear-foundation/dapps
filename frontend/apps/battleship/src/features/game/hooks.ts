import { useEffect, useMemo } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import { useLocation } from 'react-router-dom';
import { useDnsProgramIds, useProgramMetadata } from '@dapps-frontend/hooks';
import { useSignlessSendMessage } from 'gear-ez-transactions';

import meta from './assets/meta/battleship.meta.txt';
import { IGameInstance } from './types';
import { gameAtom, isActiveGameAtom, pendingAtom } from './store';
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
  const { programId } = useDnsProgramIds();

  const payload = useMemo(() => (decodedAddress ? { Game: decodedAddress } : undefined), [decodedAddress]);

  const { state: game, error } = useReadState<{ Game: IGameInstance | null }>({ programId, meta, payload });

  return { game, error };
}

export const useInitGame = () => {
  const { account } = useAccount();
  const { game, error } = useGameState();
  const { programId } = useDnsProgramIds();

  const { setGameState, resetGameState, setActiveGame } = useGame();

  useEffect(() => {
    if (!programId || !account?.decodedAddress) return;
    if (!game?.Game) return resetGameState();

    setGameState(game?.Game);
    setActiveGame(true);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress, game?.Game]);

  return {
    isGameReady: programId ? Boolean(game) : true,
    errorGame: error,
  };
};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  const { programId } = useDnsProgramIds();

  return useSignlessSendMessage(programId, metadata, { disableAlerts: true });
}

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}

export function useIsLocationGamePage() {
  const { pathname } = useLocation();

  return pathname === ROUTES.GAME;
}
