import { useEffect } from 'react';
import { useAccount, useSendMessageWithGas } from '@gear-js/react-hooks';

import { useProgramMetadata } from '@/app/hooks/use-metadata';
import meta from '@/assets/meta/vara_man.meta.txt';
import { useGame } from '@/app/context/ctx-game';
import { useApp } from '../context/ctx-app';
import { programIdGame, useGameState } from './use-game-state';
import { useNavigate } from 'react-router-dom';
import { useSignlessSendMessage } from '@dapps-frontend/ez-transactions';
import { ENV } from '../consts';

export const useInitGame = () => {
  const navigate = useNavigate()
  const { account } = useAccount();
  const { setIsSettled } = useApp();
  const { allState, config, admins, tournament } = useGameState();

  const { setTournamentGame, setIsAdmin, setConfigState, setAllGames, setPreviousGame } = useGame();

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
    if (tournament?.Tournament) {
      navigate("/")
      setTournamentGame(tournament.Tournament);
      setPreviousGame(tournament.Tournament)
    } else {
      setTournamentGame(undefined)
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [tournament?.Tournament, account?.decodedAddress])

  useEffect(() => {
    if (allState) {
      setAllGames(allState.All.tournaments)
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [allState, account?.decodedAddress])

};

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSignlessSendMessage(ENV.GAME, metadata, { disableAlerts: true });
}
