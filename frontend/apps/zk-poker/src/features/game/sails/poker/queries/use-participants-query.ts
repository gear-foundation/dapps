import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useParticipantsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'participants',
    args: [],
  });

  return { participants: data, isFetching, refetch, error };
};
