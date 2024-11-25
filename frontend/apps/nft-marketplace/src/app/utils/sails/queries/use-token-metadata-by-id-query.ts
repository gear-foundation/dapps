import { useNftProgram } from 'app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

type Params = {
  tokenId: string;
};

export const useTokenMetadataByIdQuery = ({ tokenId }: Params) => {
  const program = useNftProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, isFetched, error } = useProgramQuery({
    program,
    serviceName: 'vnft',
    functionName: 'tokenMetadataById',
    args: [tokenId],
    query: { enabled: account && tokenId ? undefined : false },
    watch: account ? true : false,
  });

  const tokenMetadata = data ? { ...data, token_id: parseInt(tokenId) } : undefined;

  return { tokenMetadata, isFetching, isFetched, refetch, error };
};
