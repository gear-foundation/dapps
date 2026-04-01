import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useLobbyGameStartTimeQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'lobbyGameStartTime',
    args: [],
  });

  return { lobbyGameStartTime: data, isFetching, refetch, error };
};
