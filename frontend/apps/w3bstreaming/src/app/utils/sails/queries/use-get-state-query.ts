import { useAccount, useProgramQuery } from '@gear-js/react-hooks';
import { useMemo } from 'react';

import { useProgram } from '@/app/utils';
import { arrayToRecord } from '@/utils';

export const useGetStateQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, isFetched, error } = useProgramQuery({
    program,
    serviceName: 'w3Bstreaming',
    functionName: 'getState',
    args: [],
    watch: account ? true : false,
  });

  const { users, streams } = useMemo(
    () => ({
      users: arrayToRecord(data?.users || []),
      streams: arrayToRecord(data?.streams || []),
    }),
    [data],
  );

  const { admins, dns_info } = data || {};

  return { users, streams, admins, dns_info, isFetching, isFetched, refetch, error };
};
