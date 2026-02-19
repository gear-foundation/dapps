import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useLobbyGameStartTimeQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'lobbyGameStartTime',
    args: [],
  });

  return { lobbyGameStartTime: data, isFetching, refetch, error };
};
