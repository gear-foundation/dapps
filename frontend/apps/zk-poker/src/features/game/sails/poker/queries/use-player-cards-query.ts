import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

type Params = {
  enabled: boolean;
};

export const usePlayerCardsQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'playerCards',
    args: [account?.decodedAddress as HexString],
    query: { enabled: !!account && enabled },
  });

  return { playerCards: data, isFetching, refetch, error };
};
