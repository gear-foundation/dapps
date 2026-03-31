import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerFactoryProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useLobbiesQuery = () => {
  const program = usePokerFactoryProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'pokerFactory',
    functionName: 'lobbies',
    args: [],
  });

  return { lobbies: castQueryData<Array<[`0x${string}`, LobbyConfig]>>(data), isFetching, refetch, error };
};
