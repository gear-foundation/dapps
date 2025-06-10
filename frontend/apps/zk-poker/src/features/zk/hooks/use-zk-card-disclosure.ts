import { useAlert } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { useEffect } from 'react';

import { useCardDisclosureMessage } from '@/features/game/sails';

import { ContractCard } from '../api/types';

const useZkCardDisclosure = (
  isWaitingForCardsToBeDisclosed: boolean,
  instances: [ContractCard, VerificationVariables][] | undefined,
) => {
  const { cardDisclosureMessage } = useCardDisclosureMessage();
  const alert = useAlert();

  const { mutateAsync } = useMutation({
    mutationFn: cardDisclosureMessage,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  useEffect(() => {
    if (isWaitingForCardsToBeDisclosed && instances) {
      void mutateAsync({ instances });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isWaitingForCardsToBeDisclosed, instances]);
};

export { useZkCardDisclosure };
