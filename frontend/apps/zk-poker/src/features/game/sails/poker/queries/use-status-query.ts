import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useStatusQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'status',
    args: [],
  });

  return { status: data, isFetching, refetch, error };
};
