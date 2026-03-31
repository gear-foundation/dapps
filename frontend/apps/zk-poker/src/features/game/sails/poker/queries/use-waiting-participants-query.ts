import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useWaitingParticipantsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'waitingParticipants',
    args: [],
  });

  return {
    waitingParticipants: castQueryData<Array<[`0x${string}`, Participant]>>(data),
    isFetching,
    refetch,
    error,
  };
};
