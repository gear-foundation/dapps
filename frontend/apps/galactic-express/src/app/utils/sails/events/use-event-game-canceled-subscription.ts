import { useProgramEvent } from '@gear-js/react-hooks';
import { useSetAtom } from 'jotai';

import { useProgram } from '@/app/utils';
import { REGISTRATION_STATUS } from '@/atoms';

export function useEventGameCanceledSubscription(isUserAdmin: boolean) {
  const program = useProgram();
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);

  const onData = () => {
    if (!isUserAdmin) {
      setRegistrationStatus('GameCanceled');
    }
  };

  useProgramEvent({
    program,
    serviceName: 'galacticExpress',
    functionName: 'subscribeToGameCanceledEvent',
    onData,
  });
}
