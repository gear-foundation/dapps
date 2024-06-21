import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useEffect, useState } from 'react';
import { isActiveGameAtom, isGameReadyAtom, multiplayerGameAtom } from '../atoms';
import { pendingAtom } from '../../game/store';
import { useMultiGameQuery } from '../sails/queries';

export function useMultiplayerGame() {
  const { account } = useAccount();
  const gameQuery = useMultiGameQuery();
  const [game, setGame] = useAtom(multiplayerGameAtom);
  const [isGameReady, setIsGameReady] = useAtom(isGameReadyAtom);
  const [isActiveGame, setIsActiveGame] = useAtom(isActiveGameAtom);
  const [error, setError] = useState<unknown | null>(null);

  const triggerGame = async () => {
    if (!account?.address) {
      return;
    }

    try {
      const res = await gameQuery(account.decodedAddress);

      setGame(res);
      if (!!res) {
        setIsActiveGame(true);
      }
      setIsGameReady(true);
    } catch (err) {
      console.log('ERrOR');
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

export function useInitMultiplayerGame() {
  const { account } = useAccount();
  const { isActiveGame, triggerGame, resetGameState } = useMultiplayerGame();
  const [isLoading, setIsLoading] = useAtom(pendingAtom);

  const initGame = async () => {
    setIsLoading(true);
    resetGameState();
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
