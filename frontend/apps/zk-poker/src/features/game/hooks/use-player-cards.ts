import { useEffect, useState } from 'react';

import { type Card } from '@/features/zk/api/types';
import { useZkKeys } from '@/features/zk/hooks';
import { decryptCards } from '@/features/zk/utils';
import { DecryptedCardsResult } from '@/features/zk/utils/decrypt-player-cards';

import { usePlayerCardsQuery } from '../sails';

// ! TODO: divide into 2 hooks for my cards and myCardsC0
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

  return {
    playerCards: decryptedCards?.cards as [Card, Card] | undefined,
    myCardsC0: decryptedCards?.myCardsC0,
    refetchPlayerCards: refetch,
  };
};

export { usePlayerCards };
