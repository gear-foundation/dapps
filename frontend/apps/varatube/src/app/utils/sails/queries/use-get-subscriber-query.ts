import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useVaratubeProgram } from '../sails';

export const useGetSubscriberQuery = () => {
  const program = useVaratubeProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error, isFetched } = useProgramQuery({
    program,
    serviceName: 'varatube',
    functionName: 'allSubscriptions',
    args: [],
    query: { enabled: true },
  });

  const subscriber = data?.find(([address]) => account?.decodedAddress === address)?.[1];

  return { subscriber, isFetching, isFetched, refetch, error };
};
