import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type RegisteredPayload = { participant_id: ActorId; pk: PublicKey };

export type Params = {
  onData: (payload: RegisteredPayload) => void;
};

export function useEventRegisteredSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToRegisteredEvent',
    onData,
  });
}
