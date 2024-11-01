import { useProgram } from '@/app/utils';
import { useProgramQuery } from '@gear-js/react-hooks';

export const useBattleQuery = (gameAddress: string) => {
  const program = useProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'battle',
    functionName: 'getBattle',
    args: [gameAddress],
    query: { enabled: false },
  });

  return { battleState: data, isFetching, refetch, error };
};
