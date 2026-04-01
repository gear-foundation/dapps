import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';

import { usePtsProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useGetBalanceQuery = () => {
  const program = usePtsProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'pts',
    functionName: 'getBalance',
    args: [account?.decodedAddress as HexString],
    query: { enabled: !!account },
  });

  return { balance: data, isFetching, refetch, error };
};
