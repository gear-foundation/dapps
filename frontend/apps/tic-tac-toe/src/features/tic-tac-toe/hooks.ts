import { useAccount } from '@gear-js/react-hooks';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import { useEffect } from 'react';

import { GameInstance } from '@/app/utils';
import { useConfigQuery, useGameQuery } from '@/features/tic-tac-toe/sails';

import { countdownAtom, gameAtom, pendingAtom } from './store';

export function useGame() {
  const setGameState = useSetAtom(gameAtom);
  const gameState = useAtomValue(gameAtom);
  const { config } = useConfigQuery();
  const setCountdown = useSetAtom(countdownAtom);
  const countdown = useAtomValue(countdownAtom);

  const updateCountdown = (game: GameInstance) => {
    setCountdown((prev) => {
      const lastTime = Number(game.last_time);
      const timeLeft = lastTime + Number(config?.turn_deadline_ms || '0');
      const isPassed = Date.now() - timeLeft > 0;
      const isNew = prev?.value !== lastTime;

      return isNew ? { value: lastTime, isActive: isNew && !isPassed } : prev;
    });
  };

  const updateGame = (game: GameInstance) => {
    setGameState(game);
    updateCountdown(game);
  };

  const clearGame = () => {
    setGameState(undefined);
    setCountdown(undefined);
  };

  const resetGame = () => {
    setGameState(null);
    setCountdown(undefined);
  };

  return {
    resetGame,
    setGameState,
    gameState,
    setCountdown,
    countdown,
    updateCountdown,
    updateGame,
    clearGame,
  };
}

export const useInitGame = () => {
  const { account } = useAccount();
  const { gameState } = useGame();

  return {
    isGameReady: account?.decodedAddress ? gameState !== undefined : true,
  };
};
export const useInitGameSync = () => {
  const { updateGame, resetGame } = useGame();
  const { game, error } = useGameQuery();

  useEffect(() => {
    if (game === undefined) return;
    game ? updateGame(game) : resetGame();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [game]);

  return {
    errorGame: error,
  };
};

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);
  return { pending, setPending };
}
