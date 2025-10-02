import { useEffect } from 'react';

import { useSubmitTablePartialDecryptionsMessage, useTableCardsToDecryptQuery } from '@/features/game/sails';

import { getZkLog, partialDecryptions } from '../utils';

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
  const { sk, pk } = useZkKeys();
  const { setLogs } = useLogs();

  useEffect(() => {
    const decrypt = async () => {
      if (!sk || !pk || !isWaitingTableCards || isDisabled) return;

      const { data: cards } = await refetchTableCardsToDecrypt();
      if (!cards?.length) return;

      const startTime = performance.now();
      const partialDecs = partialDecryptions(cards, sk, pk);
      const endTime = performance.now();
      const duration = Math.round(endTime - startTime);
      await submitTablePartialDecryptionsMessage({ partialDecs });
      setLogs((prev) => [getZkLog('🔓 Table Cards Decryption', duration), ...prev]);
    };

    void decrypt();
  }, [
    sk,
    pk,
    isWaitingTableCards,
    submitTablePartialDecryptionsMessage,
    refetchTableCardsToDecrypt,
    setLogs,
    isDisabled,
  ]);
};

export { useZkTableCardsDecryption };
