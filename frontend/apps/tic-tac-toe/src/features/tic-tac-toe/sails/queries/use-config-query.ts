import { useProgramQuery } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';

export const useConfigQuery = () => {
  const program = useProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'ticTacToe',
    functionName: 'config',
    args: [],
  });

  return { config: data, isFetching, refetch, error };
};
