import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

type Params = {
  enabled?: boolean;
};

export const useEncryptedTableCardsQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'encryptedTableCards',
    args: [],
    query: { enabled },
  });

  return { encryptedTableCards: data, isFetching, refetch, error };
};
