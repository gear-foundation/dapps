import { useProgramEvent } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export type GameRestartedPayload = { status: Status };

export type Params = {
  onData: (payload: GameRestartedPayload) => void;
};

export function useEventGameRestartedSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToGameRestartedEvent',
    onData,
  });
}
