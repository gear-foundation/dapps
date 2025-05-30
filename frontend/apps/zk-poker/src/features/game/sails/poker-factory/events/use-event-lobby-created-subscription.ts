import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerFactoryProgram } from '@/app/utils';

export type LobbyCreatedPayload = { lobby_address: ActorId; admin: ActorId; lobby_config: LobbyConfig };

export type Params = {
  onData: (payload: LobbyCreatedPayload) => void;
};

export function useEventLobbyCreatedSubscription({ onData }: Params) {
  const program = usePokerFactoryProgram();

  useProgramEvent({
    program,
    serviceName: 'pokerFactory',
    functionName: 'subscribeToLobbyCreatedEvent',
    onData,
  });
}
