import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useBettingBankQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'bettingBank',
    args: [],
  });

  return {
    bettingBank: data,
    isFetching,
    refetch,
    error,
  };
};
