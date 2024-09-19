import { useProgram } from '@/app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

export const useAllStateQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'varaMan',
    functionName: 'all',
    args: [],
    query: { enabled: account ? undefined : false },
    watch: account ? true : false,
  });

  return { allState: data, isFetching, refetch, error };
};
