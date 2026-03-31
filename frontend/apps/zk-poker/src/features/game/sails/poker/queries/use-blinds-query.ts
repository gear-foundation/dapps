import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useBlindsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'blinds',
    args: [],
  });

  return { blinds: castQueryData<[number | string | bigint, number | string | bigint]>(data), isFetching, refetch, error };
};
