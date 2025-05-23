import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { usePtsProgram } from '@/app/utils';

export const useGetBalanceQuery = () => {
  const program = usePtsProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'pts',
    functionName: 'getBalance',
    args: [account?.decodedAddress as HexString],
    query: { enabled: !!account },
  });

  return { balance: data, isFetching, refetch, error };
};
