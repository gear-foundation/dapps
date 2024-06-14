import { useEffect, useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useLocation } from 'react-router-dom';
import { useProgramMetadata } from '@dapps-frontend/hooks';
import { useSignlessSendMessage } from '@dapps-frontend/ez-transactions';
import meta from './assets/meta/battleship.meta.txt';
import { IGameInstance } from './types';
import { gameAtom, gameModeAtom, isActiveGameAtom, isGameReadyAtom, pendingAtom } from './store';
import { ADDRESS } from './consts';
import { ROUTES } from '@/app/consts';
import { sails } from '@/app/utils/sails';

export function useGame() {
  const { account } = useAccount();
  const [game, setGame] = useAtom(gameAtom);
  const [isGameReady, setIsGameReady] = useAtom(isGameReadyAtom);
  const [isActiveGame, setIsActiveGame] = useAtom(isActiveGameAtom);
  const [error, setError] = useState<unknown | null>(null);

  const triggerGame = async () => {
    if (!account?.address) {
      return;
    }

    try {
      const res = await sails.services.Single.queries.Game<IGameInstance>(
        account.address,
        undefined,
        undefined,
        account.decodedAddress,
      );

      setGame(res);

      if (!!res) {
        setIsActiveGame(true);
      }
      setIsGameReady(true);
    } catch (err) {
      setError(err);
    }
  };

  const resetGameState = () => {
    setGame(undefined);
    setIsGameReady(false);
    setIsActiveGame(false);
  };

  return { game, isActiveGame, error, isGameReady, triggerGame, resetGameState };
}

export function useInitGame() {
  const { account } = useAccount();
  const { triggerGame, resetGameState } = useGame();

  useEffect(() => {
    if (account?.decodedAddress) {
      resetGameState();
      triggerGame();
    }
  }, [account?.decodedAddress]);
}

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
