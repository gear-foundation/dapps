import { useAlert } from '@gear-js/react-hooks';
import { getErrorMessage } from '@ui/utils';
import { useEffect } from 'react';

import { useCardDisclosureMessage } from '@/features/game/sails';

import { ECPoint } from '../api/types';
import { getMyDecryptedCardsProof, getZkLog } from '../utils';

import { useLogs } from './use-logs';
import { useZkKeys } from './use-zk-keys';

const useZkCardDisclosure = (isWaitingForCardsToBeDisclosed: boolean, myCardsC0?: ECPoint[], isDisabled?: boolean) => {
  const { cardDisclosureMessage } = useCardDisclosureMessage();
  const { sk, pk } = useZkKeys();
  const alert = useAlert();
  const { setLogs } = useLogs();

  useEffect(() => {
    if (isWaitingForCardsToBeDisclosed && myCardsC0 && !isDisabled) {
      const decrypt = async () => {
        const startTime = performance.now();

        try {
          const partialDecs = getMyDecryptedCardsProof(myCardsC0, sk, pk);
          const endTime = performance.now();
          const duration = Math.round(endTime - startTime);
          await cardDisclosureMessage({ partialDecs });
          setLogs((prev) => [getZkLog('🔓 Card Disclosure', duration), ...prev]);
        } catch (error) {
          alert.error(getErrorMessage(error));
        }
      };

      void decrypt();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isWaitingForCardsToBeDisclosed, myCardsC0, isDisabled]);
};

export { useZkCardDisclosure };
