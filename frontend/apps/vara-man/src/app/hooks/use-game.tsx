import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
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
  const navigate = useNavigate();

  const { setSingleGame, setTournamentGame, setIsAdmin, setConfigState } = useGame();

  useEffect(() => {
    setConfigState(config?.Config || null);
    setIsSettled(!!config);
  }, [config?.Config]);

  useEffect(() => {
    if (!programIdGame || !account?.decodedAddress) return;

    if (admins?.Admins) {
      const isAdmin = admins.Admins.find((address) => address === account.decodedAddress);
      setIsAdmin(!!isAdmin);
    }
  }, [account?.decodedAddress, admins?.Admins]);

  useEffect(() => {
    if (game) {
      if ('SingleGame' in game && game.SingleGame) {
        setSingleGame(game.SingleGame);
      } else if ('TournamentGame' in game && game.TournamentGame) {
        setTournamentGame(game.TournamentGame);
      } else {
        setSingleGame(undefined);
        setTournamentGame(undefined)
        navigate('/')
      }
    }
  }, [game, account?.decodedAddress])

};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSendMessageHandler(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  });
}
