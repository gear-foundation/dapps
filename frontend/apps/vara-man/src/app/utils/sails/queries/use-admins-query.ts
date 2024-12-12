import { useProgram } from '@/app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

export const useAdminsQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'varaMan',
    functionName: 'admins',
    args: [],
    query: { enabled: account ? undefined : false },
  });

  return { admins: data, isFetching, refetch, error };
};
