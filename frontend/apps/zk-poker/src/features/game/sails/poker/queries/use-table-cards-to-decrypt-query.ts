import { useAccount } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

type Params = {
  enabled?: boolean;
};

export const useTableCardsToDecryptQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'tableCardsToDecrypt',
    args: [],
    query: { enabled: !!account && enabled },
  });

  return { tableCardsToDecrypt: data, isFetching, refetch, error };
};
