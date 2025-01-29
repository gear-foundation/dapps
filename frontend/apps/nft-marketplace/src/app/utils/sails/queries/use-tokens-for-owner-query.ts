import { useNftProgram } from '@/app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

export const useTokensForOwnerQuery = () => {
  const program = useNftProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, isFetched, error } = useProgramQuery({
    program,
    serviceName: 'vnft',
    functionName: 'tokensForOwner',
    args: [account?.decodedAddress!],
    query: { enabled: account ? undefined : false },
    watch: account ? true : false,
  });

  return { ownerTokens: data, isFetching, isFetched, refetch, error };
};
