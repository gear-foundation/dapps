import { HexString } from '@gear-js/api';
import { useProgramEvent, useAccount } from '@gear-js/react-hooks';
import { useSetAtom } from 'jotai';

import { useProgram } from '@/app/utils';
import { REGISTRATION_STATUS } from '@/atoms';

export function useEventPlayerDeletedSubscription() {
  const program = useProgram();
  const { account } = useAccount();
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);

  const onData = ({ player_id }: { player_id: HexString }) => {
    if (account?.decodedAddress === player_id) {
      setRegistrationStatus('PlayerRemoved');
    }
  };

  useProgramEvent({
    program,
    serviceName: 'galacticExpress',
    functionName: 'subscribeToPlayerDeletedEvent',
    onData,
  });
}
