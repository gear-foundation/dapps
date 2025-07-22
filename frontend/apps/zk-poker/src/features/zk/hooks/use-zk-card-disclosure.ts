import { useAlert } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { useEffect } from 'react';

import { useCardDisclosureMessage } from '@/features/game/sails';

import { Card, Input } from '../api/types';
import { getDecryptedCardsProof, getZkLog, logMemory } from '../utils';

import { useLogs } from './use-logs';

const useZkCardDisclosure = (
  isWaitingForCardsToBeDisclosed: boolean,
  inputs?: Input[],
  cards?: Card[],
  isDisabled?: boolean,
) => {
  const { cardDisclosureMessage } = useCardDisclosureMessage();
  const alert = useAlert();
  const { setLogs } = useLogs();

  const { mutateAsync } = useMutation({
    mutationFn: cardDisclosureMessage,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  useEffect(() => {
    if (isWaitingForCardsToBeDisclosed && inputs && cards && !isDisabled) {
      const startTime = performance.now();
      logMemory('before getDecryptedCardsProof');
      getDecryptedCardsProof(inputs, cards)
        .then(({ instances }) => mutateAsync({ instances }))
        .then(() => {
          const endTime = performance.now();
          const duration = Math.round(endTime - startTime);
          setLogs((prev) => [getZkLog('🔓 Card Disclosure', duration), ...prev]);
        })
        .then(() => {
          logMemory('after getDecryptedCardsProof');
        })
        .catch((error) => alert.error(getErrorMessage(error)));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isWaitingForCardsToBeDisclosed, inputs, cards, isDisabled]);
};

export { useZkCardDisclosure };
