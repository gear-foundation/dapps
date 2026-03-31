import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useRetiredPlayersQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'retiredPlayers',
    args: [],
  });

  return { retiredPlayers: castQueryData<Array<`0x${string}`> | null>(data), isFetching, refetch, error };
};
