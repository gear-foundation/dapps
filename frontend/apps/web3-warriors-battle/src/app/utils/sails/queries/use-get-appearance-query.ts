import { useAccount, useProgramQuery } from '@gear-js/react-hooks';

import { useWarriorProgram } from '../warrior-programm';

export const useGetAppearanceQuery = (programId: string) => {
  const program = useWarriorProgram(programId);
  const { account } = useAccount();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'warrior',
    functionName: 'getAppearance',
    args: [],
    query: { enabled: account ? undefined : false },
    watch: account ? true : false,
  });

  return { appearance: data, isFetching, refetch, error };
};
