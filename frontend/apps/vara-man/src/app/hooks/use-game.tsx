import { useEffect } from 'react';
import { useAccount, useSendMessageHandler } from '@gear-js/react-hooks';

import { useProgramMetadata } from '@/app/hooks/use-metadata';
import meta from '@/assets/meta/vara_man.meta.txt';
import { useGame } from '@/app/context/ctx-game';
import { useApp } from '../context/ctx-app';
import { programIdGame, useGameState } from './use-game-state';

export const useInitGame = () => {
  const { account } = useAccount();
  const { setIsSettled } = useApp();
  const { config, admins, game } = useGameState();

  const { setTournamentGame, setIsAdmin, setConfigState } = useGame();

  useEffect(() => {
    setConfigState(config?.Config || null);
    setIsSettled(!!config);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [config?.Config]);

  useEffect(() => {
    if (!programIdGame || !account?.decodedAddress) return;

    if (admins?.Admins) {
      const isAdmin = admins.Admins.find((address) => address === account.decodedAddress);
      setIsAdmin(!!isAdmin);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress, admins?.Admins]);

  useEffect(() => {
    if (game) {
      if ('TournamentGame' in game && game.TournamentGame) {
        setTournamentGame(game.TournamentGame);
      } else {
        setTournamentGame(undefined)
      }
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [game, account?.decodedAddress])

};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSendMessageHandler(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  });
}
