import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

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

  return { playerCards: castQueryData<EncryptedCard[] | null>(data), isFetching, refetch, error };
};
