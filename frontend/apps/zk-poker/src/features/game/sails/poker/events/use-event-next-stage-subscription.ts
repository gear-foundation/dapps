import { useProgramEvent } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export type NextStagePayload = Stage;

export type Params = {
  onData: (payload: NextStagePayload) => void;
};

export function useEventNextStageSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToNextStageEvent',
    onData,
  });
}
