import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type AdminChangedPayload = { old_admin: ActorId; new_admin: ActorId };

export type Params = {
  onData: (payload: AdminChangedPayload) => void;
};

export function useEventAdminChangedSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToAdminChangedEvent',
    onData,
  });
}
