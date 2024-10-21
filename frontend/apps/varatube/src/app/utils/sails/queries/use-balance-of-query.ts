import { useVftProgram } from '../sails';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

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
