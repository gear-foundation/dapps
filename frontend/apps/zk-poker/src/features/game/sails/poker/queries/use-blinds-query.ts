import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useBlindsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'blinds',
    args: [],
  });

  return { blinds: data, isFetching, refetch, error };
};
