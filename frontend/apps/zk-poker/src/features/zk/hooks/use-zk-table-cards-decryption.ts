import { useEffect } from 'react';

import {
  // useEventWaitingForAllTableCardsToBeDisclosedSubscription,
  // useEventWaitingForCardsToBeDisclosedSubscription,
  useSubmitTablePartialDecryptionsMessage,
  useTableCardsToDecryptQuery,
} from '@/features/game/sails';

import { partialDecryptionsForTableCards } from '../utils';

import { useKeys } from './use-keys';
import { useLogs } from './use-logs';

type Params = {
  isWaitingTableCardsAfterPreFlop?: boolean;
  isWaitingTableCardsAfterFlop?: boolean;
  isWaitingTableCardsAfterTurn?: boolean;
  isWaitingForAllTableCardsToBeDisclosed?: boolean;
  isDisabled: boolean;
  onEvent: () => void;
};

const useZkTableCardsDecryption = ({
  isWaitingTableCardsAfterPreFlop,
  isWaitingTableCardsAfterFlop,
  isWaitingTableCardsAfterTurn,
  isWaitingForAllTableCardsToBeDisclosed,
  isDisabled,
  // onEvent,
}: Params) => {
  const isWaitingTableCards =
    isWaitingTableCardsAfterPreFlop ||
    isWaitingTableCardsAfterFlop ||
    isWaitingTableCardsAfterTurn ||
    isWaitingForAllTableCardsToBeDisclosed;

  const { refetch: refetchTableCardsToDecrypt } = useTableCardsToDecryptQuery({ enabled: false });

  const { submitTablePartialDecryptionsMessage } = useSubmitTablePartialDecryptionsMessage();
  const { sk } = useKeys();
  const { setLogs } = useLogs();
  // // TODO: unused here
  // useEventWaitingForCardsToBeDisclosedSubscription({
  //   onData: () => {
  //     console.log('!!!! ~ waiting for cards to be disclosed');
  //     void refetchTableCardsToDecrypt();
  //     onEvent();
  //   },
  // });

  // // TODO: unused here
  // useEventWaitingForAllTableCardsToBeDisclosedSubscription({
  //   onData: () => {
  //     console.log('!!!! ~ waiting for all table cards to be disclosed');
  //     void refetchTableCardsToDecrypt();
  //     onEvent();
  //   },
  // });

  useEffect(() => {
    const decrypt = async () => {
      if (!sk || !isWaitingTableCards || isDisabled) return;

      const { data: cards } = await refetchTableCardsToDecrypt();
      if (!cards?.length) return;

      const startTime = performance.now();
      const decryptedCards = await partialDecryptionsForTableCards(cards, sk);
      const endTime = performance.now();
      const duration = Math.round(endTime - startTime);
      // ! TODO: remove this
      setLogs((prev) => [
        ...prev,
        `🔓 Table Cards Decryption completed in ${duration}ms (${(duration / 1000).toFixed(2)}s)`,
      ]);
      void submitTablePartialDecryptionsMessage(decryptedCards);
    };

    void decrypt();
  }, [sk, isWaitingTableCards, submitTablePartialDecryptionsMessage, refetchTableCardsToDecrypt, setLogs, isDisabled]);
};

export { useZkTableCardsDecryption };
