import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useCallback, useEffect, useState } from 'react';

import {
  useActiveParticipantsQuery,
  useBettingQuery,
  useConfigQuery,
  useParticipantsQuery,
  useTurnMessage,
} from '../sails';

import { useGetPlayerStatusAndBet } from './use-get-player-status-and-bet';

export const useTurn = () => {
  const { activeParticipants } = useActiveParticipantsQuery();
  const { participants } = useParticipantsQuery();
  const { betting } = useBettingQuery();
  const { config } = useConfigQuery();
  const { account } = useAccount();

  const { turn: contractTurn, current_bet, last_active_time, acted_players } = betting || {};

  const [currentTurn, setCurrentTurn] = useState<HexString | null>();
  const [timeToTurnEndSec, setTimeToTurnEndSec] = useState<number | null>(null);
  const [autoFoldPlayers, setAutoFoldPlayers] = useState<HexString[]>([]);
  const getPlayerStatusAndBet = useGetPlayerStatusAndBet(null, autoFoldPlayers);
  const { first_index } = activeParticipants || {};
  const participantsLength = participants?.length || 0;
  const bigBlindIndex = Number(first_index);
  const dillerIndex = (bigBlindIndex + participantsLength - 2) % participantsLength;
  const dillerAddress = participants?.[dillerIndex]?.[0];
  const activeIds = activeParticipants?.active_ids;

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

      if (!nextPlayerParticipant || autoFolded.includes(nextPlayer) || activeIds.length - autoFolded.length === 1) {
        return null;
      }

      const { status, bet } = getPlayerStatusAndBet(nextPlayer, nextPlayerParticipant[1], false);

      if (status === 'fold' || status === 'all-in') {
        return getNextActivePlayer(nextPlayer, autoFolded);
      }

      const isActed = acted_players?.find((actorId) => actorId === nextPlayer);
      // ! TODO: check if this is correct
      const isMaxBet = status === 'bet' && bet === current_bet;

      if (isActed && isMaxBet) {
        return null;
      }

      return nextPlayer || null;
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [activeIds, participants, current_bet, acted_players],
  );

  const sendAutoFoldWithRetry = useCallback((retryCount = 0) => {
    turnMessage({ action: { check: null } }).catch(() => {
      if (retryCount > 3) return;
      setTimeout(() => {
        sendAutoFoldWithRetry(retryCount + 1);
      }, 1000);
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (contractTurn && last_active_time && config && activeIds) {
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
      if (actualTurn === null && account?.decodedAddress === config?.admin_id) {
        sendAutoFoldWithRetry();
      }
      setTimeToTurnEndSec(Math.floor((timePerMoveMs - (timeLeft % timePerMoveMs)) / 1000));
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
  ]);

  const onTimeEnd = useCallback(() => {
    if (currentTurn) {
      const nextAutoFoldPlayers = [...autoFoldPlayers, currentTurn];
      const nextTurn = getNextActivePlayer(currentTurn, nextAutoFoldPlayers);
      setAutoFoldPlayers(nextAutoFoldPlayers);
      setCurrentTurn(nextTurn);
      if (!nextTurn && account?.decodedAddress === config?.admin_id) {
        sendAutoFoldWithRetry();
      }
    }
  }, [
    currentTurn,
    autoFoldPlayers,
    account?.decodedAddress,
    config?.admin_id,
    getNextActivePlayer,
    sendAutoFoldWithRetry,
  ]);

  return {
    currentTurn,
    timeToTurnEndSec,
    dillerAddress,
    autoFoldPlayers,
    getNextActivePlayer,
    onTimeEnd,
  };
};
