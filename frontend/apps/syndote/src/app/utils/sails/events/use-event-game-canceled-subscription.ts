import { useProgramEvent } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils';

export function useEventGameCanceledSubscription(onData: () => void) {
  const program = useProgram();

  useProgramEvent({
    program,
    serviceName: 'syndote',
    functionName: 'subscribeToGameWasCancelledEvent',
    onData,
  });

  useProgramEvent({
    program,
    serviceName: 'syndote',
    functionName: 'subscribeToGameDeletedEvent',
    onData,
  });
}
