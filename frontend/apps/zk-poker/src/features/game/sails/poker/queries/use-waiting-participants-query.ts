import { usePokerProgram } from '@/app/utils';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

export const useWaitingParticipantsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useTypedProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'waitingParticipants',
    args: [],
  });

  return {
    waitingParticipants: data,
    isFetching,
    refetch,
    error,
  };
};
