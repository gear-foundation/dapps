import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';

import { usePtsProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useRemainingTimeQuery = () => {
  const program = usePtsProgram();
  const { account } = useAccount();

  const { data, refetch, isPending, error } = useTypedProgramQuery({
    program,
    serviceName: 'pts',
    functionName: 'getRemainingTimeMs',
    args: [account?.decodedAddress as HexString],
    query: { enabled: !!account },
  });

  return { remainingTime: data, isPending, refetch, error };
};
