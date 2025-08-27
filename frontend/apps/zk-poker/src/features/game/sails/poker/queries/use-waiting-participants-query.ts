import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export const useWaitingParticipantsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'waitingParticipants',
    args: [],
  });

  return { waitingParticipants: data, isFetching, refetch, error };
};
