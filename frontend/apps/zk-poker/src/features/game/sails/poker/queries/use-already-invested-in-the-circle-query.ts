import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useAlreadyInvestedInTheCircleQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'alreadyInvestedInTheCircle',
    args: [],
  });

  return {
    alreadyInvestedInTheCircle: data,
    isFetching,
    refetch,
    error,
  };
};
