import { useProgramQuery } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';
import { castQueryData } from '@/features/game/sails/query-utils';

export const useActiveParticipantsQuery = () => {
  const program = usePokerProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'poker',
    functionName: 'activeParticipants',
    args: [],
  });

  return { activeParticipants: castQueryData<TurnManagerForActorId>(data), isFetching, refetch, error };
};
