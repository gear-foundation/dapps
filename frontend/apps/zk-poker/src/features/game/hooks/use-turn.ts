import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useCallback, useEffect, useRef, useState } from 'react';

import {
  useActiveParticipantsQuery,
  useAllInPlayersQuery,
  useAlreadyInvestedInTheCircleQuery,
  useBettingQuery,
  useConfigQuery,
  useParticipantsQuery,
  useTurnMessage,
} from '../sails';

import { useGetPlayerStatusAndBet } from './use-get-player-status-and-bet';

export const useTurn = (isActiveGame: boolean) => {
  const { activeParticipants } = useActiveParticipantsQuery();
  const { participants } = useParticipantsQuery();
  const { betting } = useBettingQuery();
  const { config } = useConfigQuery();
  const { account } = useAccount();
  const { alreadyInvestedInTheCircle } = useAlreadyInvestedInTheCircleQuery();
  const { allInPlayers } = useAllInPlayersQuery();

  const { turn: contractTurn, current_bet, last_active_time, acted_players } = betting || {};

  const [currentTurn, setCurrentTurn] = useState<HexString | null>();
  const [timeToTurnEndSec, setTimeToTurnEndSec] = useState<number | null>(null);
  const [autoFoldPlayers, setAutoFoldPlayers] = useState<HexString[]>([]);
  const [isTurnTimeExpired, setIsTurnTimeExpired] = useState(false);
  const autoFoldTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const isInitialLoadRef = useRef(true);
  const getPlayerStatusAndBet = useGetPlayerStatusAndBet(null, autoFoldPlayers);
  const { first_index } = activeParticipants || {};
  const participantsLength = participants?.length || 0;
  const bigBlindIndex = Number(first_index);
  const dillerIndex = (bigBlindIndex + participantsLength - 2) % participantsLength;
  const dillerAddress = participants?.[dillerIndex]?.[0];
  const activeIds = activeParticipants?.active_ids;
  const allInPlayersThisCircle = alreadyInvestedInTheCircle?.filter(([actorId]) => allInPlayers?.includes(actorId));
  const allInPlayersThisCircleLenght = allInPlayersThisCircle?.length || 0;

  const { turnMessage } = useTurnMessage(false);

  // Get next active player in turn order
  const getNextActivePlayer = useCallback(
    (currentPlayer: HexString | null, autoFolded: HexString[]): HexString | null => {
      if (!activeIds || activeIds.length === 0 || !currentPlayer) {
        return null;
      }

      const currentIndex = activeIds.indexOf(currentPlayer);
      if (currentIndex === -1) {
        return activeIds[0] || null;
      }

      // Get next player in circular order
      const nextIndex = (currentIndex + 1) % activeIds.length;

      const nextPlayer = activeIds[nextIndex];
      const nextPlayerParticipant = participants?.find(([address]) => address === nextPlayer);
      const isLastPlayerAfterAutoFold = activeIds.length + allInPlayersThisCircleLenght - autoFolded.length === 1;

      if (!nextPlayerParticipant || autoFolded.includes(nextPlayer) || isLastPlayerAfterAutoFold) {
        return null;
      }

      const { status, bet } = getPlayerStatusAndBet(nextPlayer, nextPlayerParticipant[1], false);

      if (status === 'fold' || status === 'all-in') {
        return getNextActivePlayer(nextPlayer, autoFolded);
      }

      const isActed = acted_players?.find((actorId) => actorId === nextPlayer);
      const isMaxBet = status === 'bet' && bet === current_bet;
      const isChecked = status === 'check' && current_bet === 0;

      if (isActed && (isMaxBet || isChecked)) {
        return null;
      }

      return nextPlayer || null;
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [activeIds, participants, current_bet, acted_players, allInPlayersThisCircleLenght],
  );

  const sendAutoFoldWithRetry = useCallback((retryCount = 0) => {
    turnMessage({ action: { check: null } }).catch(() => {
      if (retryCount > 3) return;
      setTimeout(() => {
        sendAutoFoldWithRetry(retryCount + 1);
      }, 2000);
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (isActiveGame && contractTurn && last_active_time && config && activeIds) {
      const timePerMoveMs = Number(config.time_per_move_ms);
      const lastActiveTime = Number(last_active_time);
      const timeLeft = Date.now() - lastActiveTime;
      const autoFoldCount = Math.min(activeIds.length, Math.floor(timeLeft / timePerMoveMs));

      let actualTurn: HexString | null = contractTurn;
      const autoFolded: HexString[] = [];
      for (let i = 0; i < autoFoldCount; i++) {
        if (actualTurn && !autoFolded.includes(actualTurn)) {
          autoFolded.push(actualTurn);
        }
        const nextTurn = getNextActivePlayer(actualTurn, autoFolded);
        actualTurn = nextTurn;
      }

      setAutoFoldPlayers(autoFolded);
      setCurrentTurn(actualTurn);
      // Trigger auto-fold only on initial page load (e.g. after reload),
      // when by this moment all turns should already be processed.
      if (isInitialLoadRef.current) {
        isInitialLoadRef.current = false;

        if (actualTurn === null && account?.decodedAddress === config?.admin_id) {
          sendAutoFoldWithRetry();
        }
      }
      const blockTimeCoverMs = 3000;
      const timeToTurnEnd = timePerMoveMs - (timeLeft % timePerMoveMs) - blockTimeCoverMs;
      setTimeToTurnEndSec(Math.floor(timeToTurnEnd / 1000));
    } else {
      setCurrentTurn(undefined);
      setTimeToTurnEndSec(null);
      setAutoFoldPlayers([]);
    }
  }, [
    contractTurn,
    last_active_time,
    config,
    activeIds,
    getNextActivePlayer,
    account?.decodedAddress,
    sendAutoFoldWithRetry,
    isActiveGame,
  ]);

  const onTimeEnd = useCallback(() => {
    // Disable UI controls immediately when time is up
    setIsTurnTimeExpired(true);

    if (autoFoldTimeoutRef.current) {
      clearTimeout(autoFoldTimeoutRef.current);
      autoFoldTimeoutRef.current = null;
    }

    if (!currentTurn) {
      return;
    }

    // Wait a bit and re-check the latest state before auto-folding
    const delayMs = 5000;

    autoFoldTimeoutRef.current = setTimeout(() => {
      const nextAutoFoldPlayers = [...autoFoldPlayers, currentTurn];
      const nextTurn = getNextActivePlayer(currentTurn, nextAutoFoldPlayers);

      setAutoFoldPlayers(nextAutoFoldPlayers);
      setCurrentTurn(nextTurn);

      if (!nextTurn && account?.decodedAddress === config?.admin_id) {
        sendAutoFoldWithRetry();
      }
    }, delayMs);
  }, [
    account?.decodedAddress,
    config?.admin_id,
    getNextActivePlayer,
    sendAutoFoldWithRetry,
    autoFoldPlayers,
    currentTurn,
  ]);

  // Reset "time expired" flag when turn changes
  useEffect(() => {
    setIsTurnTimeExpired(false);
  }, [currentTurn]);

  // Cancel pending timeout if conditions for auto-fold evaluation change
  useEffect(() => {
    return () => {
      if (autoFoldTimeoutRef.current) {
        clearTimeout(autoFoldTimeoutRef.current);
        autoFoldTimeoutRef.current = null;
      }
    };
  }, [contractTurn]);

  return {
    currentTurn,
    timeToTurnEndSec,
    dillerAddress,
    autoFoldPlayers,
    getNextActivePlayer,
    onTimeEnd,
    isTurnTimeExpired,
  };
};
