import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useBettingQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'betting',
    args: [],
  });

  return { betting: data, isFetching, refetch, error };
};
