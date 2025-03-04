import { useProgramEvent } from '@gear-js/react-hooks';

import { RoundInfo, useProgram } from '@/app/utils';

export function useEventRoundInfoSubscription(onData: (info: RoundInfo) => void) {
  const program = useProgram();

  useProgramEvent({
    program,
    serviceName: 'carRacesService',
    functionName: 'subscribeToRoundInfoEvent',
    onData,
  });
}
