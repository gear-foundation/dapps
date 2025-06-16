import { useEffect, useState } from 'react';

import { useKeys } from '@/features/zk/hooks';
import { decryptCards } from '@/features/zk/utils';
import { DecryptedCardsResult } from '@/features/zk/utils/decrypt-player-cards';

import { usePlayerCardsQuery } from '../sails';

const usePlayerCards = (enabled: boolean) => {
  const { playerCards, refetch } = usePlayerCardsQuery({ enabled });
  console.log('🚀 ~ usePlayerCards ~ playerCards:', playerCards);
  const { sk } = useKeys();

  const [decryptedCards, setDecryptedCards] = useState<DecryptedCardsResult>();

  useEffect(() => {
    if (!playerCards) return;

    void decryptCards(playerCards, sk).then((cards) => {
      setDecryptedCards(cards);
    });
  }, [playerCards, sk]);

  return { playerCards: decryptedCards?.cards, instances: decryptedCards?.instances, refetchPlayerCards: refetch };
};

export { usePlayerCards };
