import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type RegistrationCanceledPayload = { player_id: ActorId };

export type Params = {
  onData: (payload: RegistrationCanceledPayload) => void;
};

export function useEventRegistrationCanceledSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToRegistrationCanceledEvent',
    onData,
  });
}
