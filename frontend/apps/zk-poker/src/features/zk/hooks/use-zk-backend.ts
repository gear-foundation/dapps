import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useQuery } from '@tanstack/react-query';
import { useCallback, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import { getZkTask, postZkResult } from '../api';
import { DecryptOtherPlayersCardsResult, ShuffleResult } from '../api/types';
import { partialDecryptionsForPlayersCards, shuffleDeck } from '../utils';

import { useKeys } from './use-keys';

type Params = {
  isWaitingShuffleVerification: boolean;
  isWaitingPartialDecryptionsForPlayersCards: boolean;
};

const RETRY_COUNT = 20;

const useZkBackend = ({ isWaitingShuffleVerification, isWaitingPartialDecryptionsForPlayersCards }: Params) => {
  const { gameId } = useParams();
  const { account } = useAccount();
  const { sk } = useKeys();
  const alert = useAlert();

  const { data: zkTask, isLoading: isLoadingZkTask } = useQuery({
    // ! TODO: add unique key for each game
    queryKey: [
      'zk-task',
      gameId,
      account?.decodedAddress,
      isWaitingShuffleVerification,
      isWaitingPartialDecryptionsForPlayersCards,
    ],
    queryFn: () => getZkTask(gameId, account?.decodedAddress),
    enabled:
      (isWaitingShuffleVerification || isWaitingPartialDecryptionsForPlayersCards) &&
      !!account?.decodedAddress &&
      !!gameId,
    // TODO: implement ws connection instead of retry
    retry: (failureCount, error) => {
      const isNeedRetryError =
        error instanceof Error &&
        (error.message.includes('Step is not for this player') ||
          error.message.includes('Step is not for a player') ||
          error.message.includes('No step to process') ||
          error.message.includes('Player not found or game not started'));

      console.log('isNeedRetryError:', isNeedRetryError, error.message, failureCount);

      if (isNeedRetryError && failureCount < RETRY_COUNT) {
        return true;
      }
      return false;
    },
    retryDelay: 3000,
  });

  const postShuffleResult = useCallback(
    async (payload: ShuffleResult) => {
      if (!gameId || !account?.decodedAddress) throw new Error('Game ID or account is not defined');

      const res = await postZkResult({
        lobbyAddress: gameId,
        playerAddress: account.decodedAddress,
        step: 'SHUFFLE',
        result: { SHUFFLE: payload },
      });

      return res;
    },
    [gameId, account?.decodedAddress],
  );

  const postPartialDecryptionsForPlayersCardsResult = useCallback(
    async (payload: DecryptOtherPlayersCardsResult[]) => {
      if (!gameId || !account?.decodedAddress) throw new Error('Game ID or account is not defined');

      const res = await postZkResult({
        lobbyAddress: gameId,
        playerAddress: account.decodedAddress,
        step: 'DECRYPT_OTHER_PLAYERS_CARDS',
        result: { DECRYPT_OTHER_PLAYERS_CARDS: payload },
      });

      return res;
    },
    [gameId, account?.decodedAddress],
  );

  useEffect(() => {
    const postTask = async () => {
      if (!zkTask) return;

      const { SHUFFLE, DECRYPT_MY_CARDS, DECRYPT_OTHER_PLAYERS_CARDS } = zkTask.data;
      try {
        if (SHUFFLE) {
          const shuffledDeck = await shuffleDeck(SHUFFLE);
          const result = await postShuffleResult(shuffledDeck);
          console.log('postShuffleResult:', result);
        }

        if (DECRYPT_OTHER_PLAYERS_CARDS) {
          // ! TODO: remove logs
          const { otherPlayersCards } = DECRYPT_OTHER_PLAYERS_CARDS;
          const decryptedCards = await partialDecryptionsForPlayersCards(otherPlayersCards, sk);

          const result = await postPartialDecryptionsForPlayersCardsResult(decryptedCards);
          console.log('🚀 ~ postTask ~ result:', result);
        }

        if (DECRYPT_MY_CARDS) {
          // ! TODO: is this needed?
        }
      } catch (error) {
        console.error(error);
        alert.error((error as Error).message);
      }
    };

    // ! TODO: add retry on error
    void postTask();
  }, [zkTask, postShuffleResult, postPartialDecryptionsForPlayersCardsResult, alert, sk]);

  return { zkTask, isLoadingZkTask, postShuffleResult };
};

export { useZkBackend };
