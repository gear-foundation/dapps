import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

type Params = {
  enabled?: boolean;
};

export const useEncryptedTableCardsQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'encryptedTableCards',
    args: [],
    query: { enabled },
  });

  return { encryptedTableCards: data, isFetching, refetch, error };
};
