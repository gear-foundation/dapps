import { useProgramEvent } from '@gear-js/react-hooks';

import { usePokerProgram } from '@/app/utils';

export type CardsDealtToTablePayload = Array<EncryptedCard>;

export type Params = {
  onData: (payload: CardsDealtToTablePayload) => void;
};

export function useEventCardsDealtToTableSubscription({ onData }: Params) {
  const program = usePokerProgram();

  useProgramEvent({
    program,
    serviceName: 'poker',
    functionName: 'subscribeToCardsDealtToTableEvent',
    onData,
  });
}
