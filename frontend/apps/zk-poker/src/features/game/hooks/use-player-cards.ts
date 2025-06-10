import { useEffect, useState } from 'react';

import { useKeys } from '@/features/zk/hooks';
import { decryptCards } from '@/features/zk/utils';
import { DecryptedCardsResult } from '@/features/zk/utils/decrypt-player-cards';

import { usePlayerCardsQuery } from '../sails';

const usePlayerCards = (enabled: boolean) => {
  const { playerCards } = usePlayerCardsQuery({ enabled });
  const { sk } = useKeys();

  const [decryptedCards, setDecryptedCards] = useState<DecryptedCardsResult>();

  useEffect(() => {
    if (!playerCards) return;

    void decryptCards(playerCards, sk).then((cards) => {
      setDecryptedCards(cards);
    });
  }, [playerCards, sk]);

  return decryptedCards;
};

export { usePlayerCards };
