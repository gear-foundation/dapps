import { useEffect, useState } from 'react';

import { useZkKeys } from '@/features/zk/hooks';
import { decryptCards } from '@/features/zk/utils';
import { DecryptedCardsResult } from '@/features/zk/utils/decrypt-player-cards';

import { usePlayerCardsQuery } from '../sails';

const usePlayerCards = (enabled: boolean) => {
  const { playerCards, refetch } = usePlayerCardsQuery({ enabled });
  const { sk } = useZkKeys();

  const [decryptedCards, setDecryptedCards] = useState<DecryptedCardsResult>();

  useEffect(() => {
    if (!playerCards) {
      setDecryptedCards(undefined);
      return;
    }

    void decryptCards(playerCards, sk).then((cards) => {
      setDecryptedCards(cards);
    });
  }, [playerCards, sk]);

  return { playerCards: decryptedCards?.cards, inputs: decryptedCards?.inputs, refetchPlayerCards: refetch };
};

export { usePlayerCards };
