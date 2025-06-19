import { useAlert } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { useEffect } from 'react';

import { useCardDisclosureMessage } from '@/features/game/sails';

import { Card, Input } from '../api/types';
import { getDecryptedCardsProof } from '../utils';

const useZkCardDisclosure = (isWaitingForCardsToBeDisclosed: boolean, inputs?: Input[], cards?: Card[]) => {
  const { cardDisclosureMessage } = useCardDisclosureMessage();
  const alert = useAlert();

  const { mutateAsync } = useMutation({
    mutationFn: cardDisclosureMessage,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  useEffect(() => {
    if (isWaitingForCardsToBeDisclosed && inputs && cards) {
      getDecryptedCardsProof(inputs, cards)
        .then(({ instances }) => mutateAsync({ instances }))
        .catch((error) => alert.error(getErrorMessage(error)));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isWaitingForCardsToBeDisclosed, inputs, cards]);
};

export { useZkCardDisclosure };
