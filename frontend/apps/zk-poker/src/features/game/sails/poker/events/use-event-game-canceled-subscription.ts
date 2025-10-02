import { useProgramEvent } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export type Params = {
  onData: () => void;
};

export function useEventGameCanceledSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToGameCanceledEvent',
    onData,
  });
}
