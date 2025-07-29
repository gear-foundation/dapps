import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useQuery } from '@tanstack/react-query';
import { useCallback, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import { getZkTask, postZkResult } from '../api';
import { DecryptOtherPlayersCardsResult, ShuffleResult } from '../api/types';
import { getZkLog, logMemory, partialDecryptionsForPlayersCards, shuffleDeck } from '../utils';

import { useLogs } from './use-logs';
import { useZkKeys } from './use-zk-keys';

type Params = {
  isWaitingShuffleVerification: boolean;
  isWaitingPartialDecryptionsForPlayersCards: boolean;
  isDisabled: boolean;
};

const MAX_RETRY_COUNT = 30;

const useZkBackend = ({
  isWaitingShuffleVerification,
  isWaitingPartialDecryptionsForPlayersCards,
  isDisabled,
}: Params) => {
  const { gameId } = useParams();
  const { account } = useAccount();
  const { sk } = useZkKeys();
  const alert = useAlert();

  const { setLogs } = useLogs();

  const { data: zkTask, isLoading: isLoadingZkTask } = useQuery({
    queryKey: [
      'zk-task',
      gameId,
      account?.decodedAddress,
      isWaitingShuffleVerification,
      isWaitingPartialDecryptionsForPlayersCards,
      isDisabled,
    ],
    queryFn: () => getZkTask(gameId, account?.decodedAddress),
    enabled:
      (isWaitingShuffleVerification || isWaitingPartialDecryptionsForPlayersCards) &&
      !!account?.decodedAddress &&
      !!gameId &&
      !isDisabled,
    // TODO: implement ws connection instead of retry
    retry: (failureCount, error) => {
      const isNeedRetryError =
        error instanceof Error &&
        (error.message?.includes('Step is not for this player') ||
          error.message?.includes('Step is not for a player') ||
          error.message?.includes('No step to process') ||
          error.message?.includes('Player not found or game not started'));

      console.log('isNeedRetryError:', failureCount, isNeedRetryError, error.message);

      if (isNeedRetryError && failureCount < MAX_RETRY_COUNT) {
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

      logMemory('before zkTask');

      const { SHUFFLE, DECRYPT_OTHER_PLAYERS_CARDS } = zkTask.data;
      try {
        if (SHUFFLE) {
          const startTime = performance.now();
          const shuffledDeck = await shuffleDeck(SHUFFLE);
          const endTime = performance.now();
          const duration = Math.round(endTime - startTime);
          setLogs((prev) => [getZkLog('🎲 Shuffle', duration), ...prev]);

          const result = await postShuffleResult(shuffledDeck);
          console.log('postShuffleResult:', result);
        }

        if (DECRYPT_OTHER_PLAYERS_CARDS) {
          const { otherPlayersCards } = DECRYPT_OTHER_PLAYERS_CARDS;
          const startTime = performance.now();

          const decryptedCards = await partialDecryptionsForPlayersCards(otherPlayersCards, sk);

          const endTime = performance.now();
          const duration = Math.round(endTime - startTime);
          setLogs((prev) => [getZkLog('🔓 Partial Decryption', duration), ...prev]);

          const result = await postPartialDecryptionsForPlayersCardsResult(decryptedCards);
          console.log('postTask result:', result);
        }
      } catch (error) {
        console.error(error);
        alert.error((error as Error).message);
      }
    };

    void postTask();
    logMemory('after zkTask');
  }, [zkTask, postShuffleResult, postPartialDecryptionsForPlayersCardsResult, alert, sk, setLogs]);

  return {
    zkTask,
    isLoadingZkTask,
    postShuffleResult,
  };
};

export { useZkBackend };
