import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useEarnedPointsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'earnedPoints',
    args: [],
  });

  return {
    earnedPoints: data,
    isFetching,
    refetch,
    error,
  };
};
