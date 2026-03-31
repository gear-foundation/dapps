import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useAllInPlayersQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'allInPlayers',
    args: [],
  });

  return { allInPlayers: castQueryData<Array<`0x${string}`>>(data), isFetching, refetch, error };
};
