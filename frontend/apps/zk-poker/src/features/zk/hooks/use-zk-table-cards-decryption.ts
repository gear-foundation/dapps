import { useEffect } from 'react';

import {
  useEventWaitingForAllTableCardsToBeDisclosedSubscription,
  useEventWaitingForCardsToBeDisclosedSubscription,
  useSubmitTablePartialDecryptionsMessage,
  useTableCardsToDecryptQuery,
} from '@/features/game/sails';

import { partialDecryptionsForTableCards } from '../utils';

import { useKeys } from './use-keys';

type Params = {
  isWaitingTableCardsAfterPreFlop?: boolean;
  isWaitingTableCardsAfterFlop?: boolean;
  isWaitingTableCardsAfterTurn?: boolean;
  onEvent: () => void;
};

const useZkTableCardsDecryption = ({
  isWaitingTableCardsAfterPreFlop,
  isWaitingTableCardsAfterFlop,
  isWaitingTableCardsAfterTurn,
  onEvent,
}: Params) => {
  const isWaitingTableCards =
    isWaitingTableCardsAfterPreFlop || isWaitingTableCardsAfterFlop || isWaitingTableCardsAfterTurn;

  const { refetch: refetchTableCardsToDecrypt } = useTableCardsToDecryptQuery({ enabled: false });

  const { submitTablePartialDecryptionsMessage } = useSubmitTablePartialDecryptionsMessage();
  const { sk } = useKeys();

  // TODO: unused here
  useEventWaitingForCardsToBeDisclosedSubscription({
    onData: () => {
      console.log('!!!! ~ waiting for cards to be disclosed');
      void refetchTableCardsToDecrypt();
      onEvent();
    },
  });

  // TODO: unused here
  useEventWaitingForAllTableCardsToBeDisclosedSubscription({
    onData: () => {
      console.log('!!!! ~ waiting for all table cards to be disclosed');
      void refetchTableCardsToDecrypt();
      onEvent();
    },
  });

  useEffect(() => {
    const decrypt = async () => {
      if (!sk || !isWaitingTableCards) return;

      const { data: cards } = await refetchTableCardsToDecrypt();
      if (!cards?.length) return;

      const decryptedCards = await partialDecryptionsForTableCards(cards, sk);
      void submitTablePartialDecryptionsMessage(decryptedCards);
    };

    void decrypt();
  }, [sk, isWaitingTableCards, submitTablePartialDecryptionsMessage, refetchTableCardsToDecrypt]);
};

export { useZkTableCardsDecryption };
