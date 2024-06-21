import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useEffect, useState } from 'react';
import { isActiveGameAtom, isGameReadyAtom, singleGameAtom } from '../atoms';
import { useSingleGameQuery } from '../sails/queries';

export function useSingleplayerGame() {
  const { account } = useAccount();
  const gameQuery = useSingleGameQuery();
  const [game, setGame] = useAtom(singleGameAtom);
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

export function useInitSingleGame() {
  const { account } = useAccount();
  const { triggerGame, resetGameState } = useSingleplayerGame();

  useEffect(() => {
    if (account?.decodedAddress) {
      resetGameState();
      triggerGame();
    }
  }, [account?.decodedAddress]);
}
