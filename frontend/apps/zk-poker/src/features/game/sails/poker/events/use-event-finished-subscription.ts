import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type FinishedPayload = { pots: Array<[number | string | bigint, Array<ActorId>]> };

export type Params = {
  onData: (payload: FinishedPayload) => void;
};

export function useEventFinishedSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToFinishedEvent',
    onData,
  });
}
