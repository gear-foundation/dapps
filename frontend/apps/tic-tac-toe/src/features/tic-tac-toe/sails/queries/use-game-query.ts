import { useProgram } from '@/app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

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
