import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type RegisteredToTheNextRoundPayload = { participant_id: ActorId; pk: PublicKey };

export type Params = {
  onData: (payload: RegisteredToTheNextRoundPayload) => void;
};

export function useEventRegisteredToTheNextRoundSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToRegisteredToTheNextRoundEvent',
    onData,
  });
}
