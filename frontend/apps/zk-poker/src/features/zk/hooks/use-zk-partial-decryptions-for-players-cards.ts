import { useAccount, useAlert } from '@gear-js/react-hooks';
import { getErrorMessage } from '@ui/utils';
import { useEffect } from 'react';

import { useSubmitPartialDecryptionsMessage, useOtherPlayersEncryptedCardsQuery } from '@/features/game/sails';

import { getZkLog, partialDecryptions } from '../utils';

import { useLogs } from './use-logs';
import { useZkKeys } from './use-zk-keys';

const useZkPartialDecryptionsForPlayersCards = (
  isWaitingPartialDecryptionsForPlayersCards: boolean,
  isDisabled?: boolean,
) => {
  const { account } = useAccount();
  const alert = useAlert();
  const { setLogs } = useLogs();
  const { mutateAsync } = useSubmitPartialDecryptionsMessage();
  const { sk, pk } = useZkKeys();

  const { encryptedCards } = useOtherPlayersEncryptedCardsQuery({
    enabled: isWaitingPartialDecryptionsForPlayersCards,
  });

  useEffect(() => {
    if (isWaitingPartialDecryptionsForPlayersCards && encryptedCards && account && !isDisabled) {
      const decrypt = async () => {
        const startTime = performance.now();

        try {
          const partialDecs = partialDecryptions(encryptedCards, sk, pk);
          const endTime = performance.now();
          const duration = Math.round(endTime - startTime);
          await mutateAsync({ partialDecs });
          setLogs((prev) => [getZkLog('🔓 Partial Decryptions for Players Cards', duration), ...prev]);
        } catch (error) {
          alert.error(getErrorMessage(error));
        }
      };

      void decrypt();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isWaitingPartialDecryptionsForPlayersCards, encryptedCards, isDisabled]);
};

export { useZkPartialDecryptionsForPlayersCards };
