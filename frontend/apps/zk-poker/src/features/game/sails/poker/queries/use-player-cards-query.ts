import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

type Params = {
  enabled: boolean;
};

export const usePlayerCardsQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'playerCards',
    args: [account?.decodedAddress as HexString],
    query: { enabled: !!account && enabled },
  });

  return { playerCards: data, isFetching, refetch, error };
};
