import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useAlreadyInvestedInTheCircleQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'alreadyInvestedInTheCircle',
    args: [],
  });

  return { alreadyInvestedInTheCircle: data, isFetching, refetch, error };
};
