import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

import { useProgramEvent } from './use-program-event';

export type RegistrationCanceledPayload = { player_id: ActorId };

export type Params = {
  onData: (payload: RegistrationCanceledPayload) => void;
  queryKey?: unknown[];
};

export function useEventRegistrationCanceledSubscription({ onData, queryKey }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToRegistrationCanceledEvent',
    onData,
    queryKey,
  });
}
