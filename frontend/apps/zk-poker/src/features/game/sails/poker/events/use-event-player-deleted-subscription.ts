import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type PlayerDeletedPayload = { player_id: ActorId };

export type Params = {
  onData: (payload: PlayerDeletedPayload) => void;
};

export function useEventPlayerDeletedSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToPlayerDeletedEvent',
    onData,
  });
}
