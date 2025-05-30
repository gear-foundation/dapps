import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerFactoryProgram } from '@/app/utils';

export const useLobbiesQuery = () => {
  const program = usePokerFactoryProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'pokerFactory',
    functionName: 'lobbies',
    args: [],
  });

  return { lobbies: data, isFetching, refetch, error };
};
