import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export const useConfigQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'battle',
    functionName: 'config',
    args: [],
    query: { enabled: account ? undefined : false },
  });

  return { config: data, isFetching, refetch, error };
};
