import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { useGame } from '@/app/context/ctx-game';
import { useDnsProgramIds } from '@dapps-frontend/hooks';
import { useAdminsQuery, useAllStateQuery, useConfigQuery, useTournamentQuery } from '../utils';

export const useInitGame = () => {
  const navigate = useNavigate();
  const { account } = useAccount();

  const { admins } = useAdminsQuery();
  const { allState } = useAllStateQuery();
  const { config } = useConfigQuery();
  const { tournament } = useTournamentQuery();

  const { programId } = useDnsProgramIds();

  const { setTournamentGame, setIsAdmin, setConfigState, setAllGames, setPreviousGame } = useGame();

  useEffect(() => {
    setConfigState(config || null);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [config]);

  useEffect(() => {
    if (!programId || !account?.decodedAddress) return;

    if (admins) {
      const isAdmin = admins.find((address) => address === account.decodedAddress);
      setIsAdmin(!!isAdmin);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress, admins]);

  useEffect(() => {
    if (tournament) {
      navigate('/');
      setTournamentGame(tournament);
      setPreviousGame(tournament);
    } else {
      setTournamentGame(undefined);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [tournament, account?.decodedAddress]);

  useEffect(() => {
    if (allState) {
      setAllGames(allState.tournaments);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [allState, account?.decodedAddress]);
};
