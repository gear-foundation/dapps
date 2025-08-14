import { useProgramEvent } from '@gear-js/react-hooks';

import { usePokerFactoryProgram } from '@/app/utils';

export type Params = {
  onData: () => void;
};

export function useEventLobbyDeletedSubscription({ onData }: Params) {
  const program = usePokerFactoryProgram();

  useProgramEvent({
    program,
    serviceName: 'pokerFactory',
    functionName: 'subscribeToLobbyDeletedEvent',
    onData,
  });
}
