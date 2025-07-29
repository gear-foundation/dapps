import { useProgramEvent } from '@gear-js/react-hooks';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export type CardsDealtToPlayersPayload = Array<[ActorId, [EncryptedCard, EncryptedCard]]>;

export type Params = {
  onData: (payload: CardsDealtToPlayersPayload) => void;
};

export function useEventCardsDealtToPlayersSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToCardsDealtToPlayersEvent',
    onData,
  });
}
