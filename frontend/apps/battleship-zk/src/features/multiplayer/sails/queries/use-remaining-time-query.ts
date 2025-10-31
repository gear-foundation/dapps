import { useAccount, useProgramQuery } from '@gear-js/react-hooks';
import { useEffect } from 'react';

import { useProgram } from '@/app/utils/sails';

import { useMultiplayerGame } from '../../hooks';
import { SERVICE_NAME } from '../consts';

export const useRemainingTimeQuery = () => {
  const { account } = useAccount();
  const { game, gameEndResult } = useMultiplayerGame();
  const program = useProgram();
  const address = account?.decodedAddress || '';
  const { data, isFetching, refetch } = useProgramQuery({
    program,
    serviceName: SERVICE_NAME,
    functionName: 'getRemainingTime',
    args: [address],
    query: { enabled: false },
  });

  useEffect(() => {
    void refetch();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [game?.last_move_time]);

  const remainingTime = gameEndResult && gameEndResult.winner !== address ? 0 : data;

  return isFetching ? undefined : remainingTime;
};
