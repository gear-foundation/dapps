import { useAccount, useAlert } from '@gear-js/react-hooks';
import { getErrorMessage } from '@ui/utils';
import { useEffect, useState } from 'react';

import {
  useEventCardsDealtToPlayersSubscription,
  useSubmitPartialDecryptionsMessage,
  type CardsDealtToPlayersPayload,
  useEventAllPartialDecryptionsSubmitedSubscription,
} from '@/features/game/sails';

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

  const [allPlayersEncryptedCards, setAllPlayersEncryptedCards] = useState<CardsDealtToPlayersPayload | null>(null);

  useEventCardsDealtToPlayersSubscription({
    onData: (payload) => {
      setAllPlayersEncryptedCards(payload);
    },
  });

  useEventAllPartialDecryptionsSubmitedSubscription({
    onData: () => {
      setAllPlayersEncryptedCards(null);
    },
  });

  useEffect(() => {
    if (isWaitingPartialDecryptionsForPlayersCards && allPlayersEncryptedCards && account && !isDisabled) {
      const decrypt = async () => {
        const startTime = performance.now();
        const otherPlayersEncryptedCards = allPlayersEncryptedCards
          .filter(([playerAddress]) => playerAddress !== account.decodedAddress)
          .flatMap(([_, [card1, card2]]) => [card1, card2]);

        try {
          const partialDecs = partialDecryptions(otherPlayersEncryptedCards, sk, pk);
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
  }, [isWaitingPartialDecryptionsForPlayersCards, allPlayersEncryptedCards, isDisabled]);
};

export { useZkPartialDecryptionsForPlayersCards };
