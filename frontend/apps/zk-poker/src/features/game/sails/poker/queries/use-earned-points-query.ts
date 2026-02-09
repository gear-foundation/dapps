import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useEarnedPointsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'earnedPoints',
    args: [],
  });

  return { earnedPoints: data, isFetching, refetch, error };
};
