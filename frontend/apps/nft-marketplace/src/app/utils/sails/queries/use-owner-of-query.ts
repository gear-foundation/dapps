import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useNftProgram } from '@/app/utils';

type Params = {
  tokenId: string;
};

export const useOwnerOfQuery = ({ tokenId }: Params) => {
  const program = useNftProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, isFetched, error } = useProgramQuery({
    program,
    serviceName: 'vnft',
    functionName: 'ownerOf',
    args: [tokenId],
    query: { enabled: tokenId ? undefined : false },
    watch: account ? true : false,
  });

  return { owner: data, isFetching, isFetched, refetch, error };
};
