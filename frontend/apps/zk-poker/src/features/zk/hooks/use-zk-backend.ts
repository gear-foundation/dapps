import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useCallback, useEffect, useState, useRef } from 'react';
import { useParams } from 'react-router-dom';

import {
  subscribeToTasks,
  submitResult,
  unsubscribeFromTasks,
  setWebSocketEventHandlers,
  isWebSocketConnected,
  reconnectWebSocket,
  type ZkTaskResponse,
  type SubscriptionStatusResponse,
  type ResultProcessedResponse,
  type WebSocketError,
} from '../api';
import { DecryptOtherPlayersCardsResult, GameProgressEvent, ShuffleResult } from '../api/types';
import { getZkLog, logMemory, partialDecryptionsForPlayersCards, shuffleDeck } from '../utils';

import { useLogs } from './use-logs';
import { useZkKeys } from './use-zk-keys';

type Params = {
  isWaitingShuffleVerification: boolean;
  isWaitingPartialDecryptionsForPlayersCards: boolean;
  isDisabled: boolean;
};

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

  // WebSocket state
  const [zkTask, setZkTask] = useState<ZkTaskResponse | null>(null);
  const [isLoadingZkTask, setIsLoadingZkTask] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [subscriptionStatus, setSubscriptionStatus] = useState<SubscriptionStatusResponse | null>(null);
  const [gameProgress, setGameProgress] = useState<GameProgressEvent>();
  const isSubscribedRef = useRef(false);
  const shouldBeSubscribedRef = useRef(false);

  // WebSocket event handlers
  const handleNewTask = useCallback((task: ZkTaskResponse) => {
    console.log('Received new task via WebSocket:', task);
    setZkTask(task);
    setIsLoadingZkTask(false);
  }, []);

  const handleSubscriptionStatus = useCallback((status: SubscriptionStatusResponse) => {
    console.log('Subscription status:', status);
    setSubscriptionStatus(status);
    isSubscribedRef.current = status?.subscribed || false;
  }, []);

  const handleResultProcessed = useCallback(
    (response: ResultProcessedResponse) => {
      console.log('Result processed:', response);
      if (!response?.success && response?.message) {
        alert.error(`Result processing failed: ${response.message}`);
      }
    },
    [alert],
  );

  const handleWebSocketError = useCallback(
    (error: WebSocketError) => {
      console.error('WebSocket error:', error);
      alert.error(`WebSocket error: ${error?.message || 'Unknown error'}`);
      setIsLoadingZkTask(false);
    },
    [alert],
  );

  const handleConnect = useCallback(() => {
    console.log('WebSocket connected');
    setIsConnected(true);
  }, []);

  const handleDisconnect = useCallback(() => {
    console.log('WebSocket disconnected');
    setIsConnected(false);
    isSubscribedRef.current = false;
    setSubscriptionStatus(null);
  }, []);

  const postShuffleResult = useCallback(
    async (payload: ShuffleResult) => {
      if (!gameId || !account?.decodedAddress) throw new Error('Game ID or account is not defined');

      await submitResult({
        lobbyAddress: gameId,
        playerAddress: account.decodedAddress,
        step: 'SHUFFLE',
        result: { SHUFFLE: payload },
      });
    },
    [gameId, account?.decodedAddress],
  );

  const postPartialDecryptionsForPlayersCardsResult = useCallback(
    async (payload: DecryptOtherPlayersCardsResult[]) => {
      if (!gameId || !account?.decodedAddress) throw new Error('Game ID or account is not defined');

      await submitResult({
        lobbyAddress: gameId,
        playerAddress: account.decodedAddress,
        step: 'DECRYPT_OTHER_PLAYERS_CARDS',
        result: { DECRYPT_OTHER_PLAYERS_CARDS: payload },
      });
    },
    [gameId, account?.decodedAddress],
  );

  const handleGameProgress = useCallback((progress: GameProgressEvent) => {
    console.log('Game progress received:', progress);
    setGameProgress(progress);
  }, []);

  // Set up WebSocket event handlers
  useEffect(() => {
    setWebSocketEventHandlers({
      onNewTask: handleNewTask,
      onSubscriptionStatus: handleSubscriptionStatus,
      onResultProcessed: handleResultProcessed,
      onError: handleWebSocketError,
      onConnect: handleConnect,
      onDisconnect: handleDisconnect,
      onGameProgress: handleGameProgress,
    });

    // Initialize connection state
    setIsConnected(isWebSocketConnected());
  }, [
    handleNewTask,
    handleSubscriptionStatus,
    handleResultProcessed,
    handleWebSocketError,
    handleConnect,
    handleDisconnect,
    handleGameProgress,
  ]);

  // Handle subscription logic
  useEffect(() => {
    const shouldSubscribe =
      (isWaitingShuffleVerification || isWaitingPartialDecryptionsForPlayersCards) &&
      !!account?.decodedAddress &&
      !!gameId &&
      !isDisabled;

    shouldBeSubscribedRef.current = shouldSubscribe;

    const handleSubscription = async () => {
      if (!gameId || !account?.decodedAddress) return;

      try {
        if (shouldSubscribe && isConnected && !isSubscribedRef.current) {
          console.log('Subscribing to tasks...');
          setIsLoadingZkTask(true);
          await subscribeToTasks({
            lobbyAddress: gameId,
            playerAddress: account.decodedAddress,
          });
        } else if (!shouldSubscribe && isSubscribedRef.current) {
          console.log('Unsubscribing from tasks...');
          await unsubscribeFromTasks({
            lobbyAddress: gameId,
            playerAddress: account.decodedAddress,
          });
          setZkTask(null);
          setIsLoadingZkTask(false);
        }
      } catch (error) {
        console.error('Subscription error:', error);
        alert.error(`Subscription error: ${(error as Error).message}`);
        setIsLoadingZkTask(false);
      }
    };

    void handleSubscription();
  }, [
    isWaitingShuffleVerification,
    isWaitingPartialDecryptionsForPlayersCards,
    isDisabled,
    gameId,
    account?.decodedAddress,
    isConnected,
    alert,
  ]);

  // Handle reconnection
  useEffect(() => {
    if (!isConnected && shouldBeSubscribedRef.current) {
      const reconnectTimer = setTimeout(() => {
        console.log('Attempting to reconnect WebSocket...');
        reconnectWebSocket();
      }, 3000);

      return () => clearTimeout(reconnectTimer);
    }
  }, [isConnected]);

  // Process tasks when received
  useEffect(() => {
    const processTask = async () => {
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

          await postShuffleResult(shuffledDeck);
          console.log('Shuffle result submitted via WebSocket');
        }

        if (DECRYPT_OTHER_PLAYERS_CARDS) {
          const { otherPlayersCards } = DECRYPT_OTHER_PLAYERS_CARDS;
          const startTime = performance.now();

          const decryptedCards = await partialDecryptionsForPlayersCards(otherPlayersCards, sk);

          const endTime = performance.now();
          const duration = Math.round(endTime - startTime);
          setLogs((prev) => [getZkLog('🔓 Partial Decryption', duration), ...prev]);

          await postPartialDecryptionsForPlayersCardsResult(decryptedCards);
          console.log('Partial decryption result submitted via WebSocket');
        }
      } catch (error) {
        console.error('Task processing error:', error);
        alert.error((error as Error).message);
      }
    };

    void processTask();
    logMemory('after zkTask');
  }, [zkTask, postShuffleResult, postPartialDecryptionsForPlayersCardsResult, alert, sk, setLogs]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (isSubscribedRef.current && gameId && account?.decodedAddress) {
        unsubscribeFromTasks({
          lobbyAddress: gameId,
          playerAddress: account.decodedAddress,
        }).catch(console.error);
      }
    };
  }, [gameId, account?.decodedAddress]);

  return {
    zkTask,
    isLoadingZkTask,
    isConnected,
    subscriptionStatus: subscriptionStatus || null,
    postShuffleResult,
    postPartialDecryptionsForPlayersCardsResult,
    gameProgress,
  };
};

export { useZkBackend };
