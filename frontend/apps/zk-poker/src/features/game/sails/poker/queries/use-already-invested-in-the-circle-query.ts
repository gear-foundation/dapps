import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useAlreadyInvestedInTheCircleQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'alreadyInvestedInTheCircle',
    args: [],
  });

  return {
    alreadyInvestedInTheCircle: castQueryData<Array<[`0x${string}`, number | string | bigint]>>(data),
    isFetching,
    refetch,
    error,
  };
};
