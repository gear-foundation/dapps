import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type FinishedPayload = { winners: ActorId[]; cash_prize: Array<number | string | bigint> };

export type Params = {
  onData: (payload: FinishedPayload) => void;
};

export function useEventGameEndedSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToFinishedEvent',
    onData,
  });
}
