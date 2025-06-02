import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export const useGameQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'ticTacToe',
    functionName: 'game',
    args: [account?.decodedAddress!],
    query: { enabled: account ? undefined : false },
  });

  return { game: data, isFetching, refetch, error };
};
