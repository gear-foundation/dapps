import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useLobbyGameStartTimeQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'lobbyGameStartTime',
    args: [],
  });

  return { lobbyGameStartTime: castQueryData<bigint>(data), isFetching, refetch, error };
};
