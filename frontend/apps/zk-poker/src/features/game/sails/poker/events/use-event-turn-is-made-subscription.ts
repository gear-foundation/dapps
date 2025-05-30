import { useProgramEvent } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export type TurnIsMadePayload = { action: Action };

export type Params = {
  onData: (payload: TurnIsMadePayload) => void;
};

export function useEventTurnIsMadeSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToTurnIsMadeEvent',
    onData,
  });
}
