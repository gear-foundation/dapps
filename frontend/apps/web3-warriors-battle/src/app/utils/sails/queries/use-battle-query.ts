import { useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export const useBattleQuery = (gameAddress: string | null) => {
  const program = useProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'battle',
    functionName: 'getBattle',
    args: [gameAddress || ''],
    query: { enabled: false },
  });

  return { battleState: data, isFetching, refetch, error };
};
