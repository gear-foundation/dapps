import { useProgramEvent } from '@gear-js/react-hooks';
import { useProgram } from 'app/utils';
import { Step } from 'types';

export function useEventStepSubscription(onData: (data: Step) => void) {
  const program = useProgram();

  useProgramEvent({
    program,
    serviceName: 'syndote',
    functionName: 'subscribeToStepEvent',
    onData,
  });
}
