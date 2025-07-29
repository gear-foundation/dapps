import { useEffect } from 'react';

import { useSubmitTablePartialDecryptionsMessage, useTableCardsToDecryptQuery } from '@/features/game/sails';

import { getZkLog, logMemory, partialDecryptionsForTableCards } from '../utils';

import { useLogs } from './use-logs';
import { useZkKeys } from './use-zk-keys';

type Params = {
  isWaitingTableCardsAfterPreFlop?: boolean;
  isWaitingTableCardsAfterFlop?: boolean;
  isWaitingTableCardsAfterTurn?: boolean;
  isWaitingForAllTableCardsToBeDisclosed?: boolean;
  isDisabled: boolean;
};

const useZkTableCardsDecryption = ({
  isWaitingTableCardsAfterPreFlop,
  isWaitingTableCardsAfterFlop,
  isWaitingTableCardsAfterTurn,
  isWaitingForAllTableCardsToBeDisclosed,
  isDisabled,
}: Params) => {
  const isWaitingTableCards =
    isWaitingTableCardsAfterPreFlop ||
    isWaitingTableCardsAfterFlop ||
    isWaitingTableCardsAfterTurn ||
    isWaitingForAllTableCardsToBeDisclosed;

  const { refetch: refetchTableCardsToDecrypt } = useTableCardsToDecryptQuery({ enabled: false });

  const { submitTablePartialDecryptionsMessage } = useSubmitTablePartialDecryptionsMessage();
  const { sk } = useZkKeys();
  const { setLogs } = useLogs();

  useEffect(() => {
    const decrypt = async () => {
      if (!sk || !isWaitingTableCards || isDisabled) return;

      const { data: cards } = await refetchTableCardsToDecrypt();
      if (!cards?.length) return;

      logMemory('before partialDecryptionsForTableCards');
      const startTime = performance.now();
      const decryptedCards = await partialDecryptionsForTableCards(cards, sk);
      const endTime = performance.now();
      const duration = Math.round(endTime - startTime);
      setLogs((prev) => [getZkLog('🔓 Table Cards Decryption', duration), ...prev]);
      logMemory('after partialDecryptionsForTableCards');
      await submitTablePartialDecryptionsMessage(decryptedCards);
      logMemory('after submitTablePartialDecryptionsMessage');
    };

    void decrypt();
  }, [sk, isWaitingTableCards, submitTablePartialDecryptionsMessage, refetchTableCardsToDecrypt, setLogs, isDisabled]);
};

export { useZkTableCardsDecryption };
