import { useMarketplaceProgram } from 'app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

export const useGetMarketQuery = () => {
  const program = useMarketplaceProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, isFetched, error } = useProgramQuery({
    program,
    serviceName: 'nftMarketplace',
    functionName: 'getMarket',
    args: [],
    watch: account ? true : false,
  });

  return { market: data, isFetching, isFetched, refetch, error };
};
