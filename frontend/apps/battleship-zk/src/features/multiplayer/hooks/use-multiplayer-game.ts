import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useEffect, useState } from 'react';
import { gameEndResultAtom, isActiveGameAtom, isGameReadyAtom, multiplayerGameAtom } from '../atoms';
import { useMultiGameQuery } from '../sails/queries';

export function useMultiplayerGame() {
  const { account } = useAccount();
  const gameQuery = useMultiGameQuery();
  const [game, setGame] = useAtom(multiplayerGameAtom);
  const [isGameReady, setIsGameReady] = useAtom(isGameReadyAtom);
  const [isActiveGame, setIsActiveGame] = useAtom(isActiveGameAtom);
  const [gameEndResult, setGameEndResult] = useAtom(gameEndResultAtom);

  const [error, setError] = useState<unknown | null>(null);

  const triggerGame = async () => {
    if (!account?.address) {
      return;
    }

    try {
      const res = await gameQuery(account.decodedAddress);

      setGame(res);
      if (res) {
        setIsActiveGame(true);
      }
      setIsGameReady(true);
    } catch (err) {
      console.log(err);
      setError(err);
    }
  };

  const resetGameState = () => {
    setGame(undefined);
    setIsGameReady(false);
    setIsActiveGame(false);
  };

  return { game, isActiveGame, error, isGameReady, triggerGame, resetGameState, gameEndResult, setGameEndResult };
}

export function useInitMultiplayerGame() {
  const { account } = useAccount();
  const { isActiveGame, triggerGame, resetGameState, setGameEndResult } = useMultiplayerGame();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const initGame = async () => {
    setIsLoading(true);
    resetGameState();
    setGameEndResult(null);
    await triggerGame();
    setIsLoading(false);
  };

  useEffect(() => {
    if (account?.decodedAddress) {
      initGame();
    }
  }, [account?.decodedAddress]);

  return { isLoading, isActiveGame };
}
