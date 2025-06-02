import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export const useGetGameQuery = (gameAddress?: HexString) => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'galacticExpress',
    functionName: 'getGame',
    args: [gameAddress || '0x'],
    query: { enabled: account && gameAddress ? undefined : false },
    watch: true,
  });

  return { game: data, isFetching, refetch, error };
};
