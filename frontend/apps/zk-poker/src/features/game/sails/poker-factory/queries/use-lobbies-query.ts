import { usePokerFactoryProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useLobbiesQuery = () => {
  const program = usePokerFactoryProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'pokerFactory',
    functionName: 'lobbies',
    args: [],
  });

  return { lobbies: data, isFetching, refetch, error };
};
