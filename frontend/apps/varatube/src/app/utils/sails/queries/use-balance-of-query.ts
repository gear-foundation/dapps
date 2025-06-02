import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useVftProgram } from '../sails';

export const useBalanceOfQuery = () => {
  const program = useVftProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'balanceOf',
    args: [account?.decodedAddress || ''],
    query: { enabled: account ? undefined : false },
  });

  return { balance: data, isFetching, refetch, error };
};
