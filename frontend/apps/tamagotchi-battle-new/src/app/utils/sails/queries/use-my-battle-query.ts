import { useProgram } from '@/app/utils';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

export const useMyBattleQuery = () => {
  const program = useProgram();
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'battle',
    functionName: 'getMyBattle',
    args: [],
    query: { enabled: account ? undefined : false },
    watch: account ? true : false,
  });

  return { battleState: data, isFetching, refetch, error };
};
