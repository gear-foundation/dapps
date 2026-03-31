import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

type Params = {
  enabled?: boolean;
};

export const useRevealedPlayersQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'revealedPlayers',
    args: [],
    query: { enabled: !!account && enabled },
  });

  return {
    revealedPlayers: castQueryData<Array<[`0x${string}`, [globalThis.Card, globalThis.Card]]>>(data),
    isFetching,
    refetch,
    error,
  };
};
