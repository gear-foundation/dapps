import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export const useTournamentQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'varaMan',
    functionName: 'getTournament',
    args: [account?.decodedAddress!],
    query: { enabled: account ? undefined : false },
    watch: account ? true : false,
  });

  return { tournament: data?.[0], isFetching, refetch, error };
};
