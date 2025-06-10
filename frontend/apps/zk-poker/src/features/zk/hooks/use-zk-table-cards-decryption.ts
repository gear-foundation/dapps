import { useEffect } from 'react';

import { useEncryptedTableCardsQuery, useSubmitTablePartialDecryptionsMessage } from '@/features/game/sails';

import { partialDecryptionsForTableCards } from '../utils';

import { useKeys } from './use-keys';

type Params = {
  isWaitingTableCardsAfterPreFlop?: boolean;
  isWaitingTableCardsAfterFlop?: boolean;
  isWaitingTableCardsAfterTurn?: boolean;
};

const useZkTableCardsDecryption = ({
  isWaitingTableCardsAfterPreFlop,
  isWaitingTableCardsAfterFlop,
  isWaitingTableCardsAfterTurn,
}: Params) => {
  const isWaitingTableCards =
    isWaitingTableCardsAfterPreFlop || isWaitingTableCardsAfterFlop || isWaitingTableCardsAfterTurn;

  const { encryptedTableCards } = useEncryptedTableCardsQuery({ enabled: isWaitingTableCards });
  const { submitTablePartialDecryptionsMessage } = useSubmitTablePartialDecryptionsMessage();
  const { sk } = useKeys();

  useEffect(() => {
    const decrypt = async () => {
      if (encryptedTableCards && sk && isWaitingTableCards) {
        const getActualCards = () => {
          // ! TODO: it will change. Only actual cards will be returned from contract
          if (isWaitingTableCardsAfterPreFlop) {
            return encryptedTableCards.slice(0, 3);
          }
          if (isWaitingTableCardsAfterFlop) {
            return encryptedTableCards.slice(3, 4);
          }
          return encryptedTableCards.slice(4, 5);
        };

        const decryptedCards = await partialDecryptionsForTableCards(getActualCards(), sk);
        void submitTablePartialDecryptionsMessage(decryptedCards);
      }
    };

    void decrypt();
  }, [
    encryptedTableCards,
    sk,
    isWaitingTableCards,
    submitTablePartialDecryptionsMessage,
    isWaitingTableCardsAfterPreFlop,
    isWaitingTableCardsAfterFlop,
    isWaitingTableCardsAfterTurn,
  ]);
};

export { useZkTableCardsDecryption };
