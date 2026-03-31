import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { usePtsProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useRemainingTimeQuery = () => {
  const program = usePtsProgram();
  const { account } = useAccount();

  const { data, refetch, isPending, error } = useProgramQuery({
    program,
    serviceName: 'pts',
    functionName: 'getRemainingTimeMs',
    args: [account?.decodedAddress as HexString],
    query: { enabled: !!account },
  });

  return { remainingTime: castQueryData<number | string | bigint | null>(data), isPending, refetch, error };
};
