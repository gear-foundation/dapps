import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export const useAdminsQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'battle',
    functionName: 'admins',
    args: [],
    query: { enabled: account ? undefined : false },
  });

  return { admins: data, isFetching, refetch, error };
};
