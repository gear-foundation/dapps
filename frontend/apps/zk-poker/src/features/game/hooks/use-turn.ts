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
  const [timeToTurnEnd, setTimeToTurnEnd] = useState<number | null>(null);
  const [autoFoldPlayers, setAutoFoldPlayers] = useState<HexString[]>([]);
  const getPlayerStatusAndBet = useGetPlayerStatusAndBet(null, autoFoldPlayers);
  // ! TODO: check if this is correct ( - 2 or another number)
  const { first_index, turn_index } = activeParticipants || {};

  const dillerIndex = (Number(first_index) + Number(turn_index) - 2) % (participants?.length || 0);
  const dillerAddress = participants?.[dillerIndex]?.[0];
  const activeIds = activeParticipants?.active_ids;

  const { turnMessage } = useTurnMessage();

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

      if (!nextPlayerParticipant || autoFolded.includes(nextPlayer)) {
        return null;
      }

      const { status, bet } = getPlayerStatusAndBet(nextPlayer, nextPlayerParticipant[1], false);

      if (status === 'fold' || status === 'all-in') {
        return getNextActivePlayer(nextPlayer, autoFolded);
      }

      const isActed = acted_players?.find((actorId) => actorId === nextPlayer);
      // ! TODO: check if this is correct
      // const isActed = (status === 'bet' && bet === current_bet) || (current_bet === 0 && status === 'check');
      const isMaxBet = status === 'bet' && bet === current_bet;

      if (isActed && isMaxBet) {
        return null;
      }

      return nextPlayer || null;
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [activeIds, participants, current_bet, acted_players],
  );

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
        console.log('!!!!!!!!!!!!! last turn ended');
        void turnMessage({ action: { check: null } });
      }
      setTimeToTurnEnd(timeLeft % timePerMoveMs);
    } else {
      setCurrentTurn(undefined);
      setTimeToTurnEnd(null);
    }
  }, [contractTurn, last_active_time, config, activeIds, getNextActivePlayer, account?.decodedAddress, turnMessage]);

  const onTimeEnd = () => {
    if (currentTurn) {
      const nextTurn = getNextActivePlayer(currentTurn, autoFoldPlayers);
      setAutoFoldPlayers((prev) => [...prev, currentTurn]);
      setCurrentTurn(nextTurn);
      if (!nextTurn && account?.decodedAddress === config?.admin_id) {
        console.log('!!!!!!!!!!!!! last turn ended');
        void turnMessage({ action: { check: null } });
      }
    }
  };

  return {
    currentTurn,
    timeToTurnEnd,
    dillerAddress,
    autoFoldPlayers,
    getNextActivePlayer,
    onTimeEnd,
  };
};
