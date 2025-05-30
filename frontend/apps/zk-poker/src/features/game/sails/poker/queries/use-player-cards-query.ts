import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const usePlayerCardsQuery = () => {
  const program = usePokerProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'playerCards',
    args: [account?.address as HexString],
    query: { enabled: !!account },
  });

  return { playerCards: data, isFetching, refetch, error };
};
