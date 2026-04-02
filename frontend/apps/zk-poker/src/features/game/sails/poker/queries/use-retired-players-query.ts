import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useRetiredPlayersQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'retiredPlayers',
    args: [],
  });

  return { retiredPlayers: data, isFetching, refetch, error };
};
