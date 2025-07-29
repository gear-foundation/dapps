import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useCallback } from 'react';

import { PlayerStatus } from '@/features/game/types';

import {
  useActiveParticipantsQuery,
  useAlreadyInvestedInTheCircleQuery,
  useBettingBankQuery,
  useBettingQuery,
} from '../sails';

const useGetPlayerStatusAndBet = (turn: HexString | null, autoFoldPlayers: HexString[]) => {
  const { account } = useAccount();
  const { alreadyInvestedInTheCircle } = useAlreadyInvestedInTheCircleQuery();
  const { activeParticipants } = useActiveParticipantsQuery();
  const { bettingBank } = useBettingBankQuery();
  const { betting } = useBettingQuery();
  const getPlayerStatusAndBet = useCallback(
    (
      address: HexString,
      participant: Participant,
      isActiveGame: boolean,
      pots?: [string | number | bigint, HexString[]][],
    ): { status: PlayerStatus; bet?: number } => {
      const investedInTheCircle = alreadyInvestedInTheCircle?.find(([actorId]) => actorId === address);

      if (autoFoldPlayers.includes(address)) {
        return { status: 'fold' };
      }

      if (pots?.some(([_, winners]) => winners.includes(address))) {
        return { status: 'winner' };
      }

      const isHaveNoBalance = participant.balance === 0;
      const isHaveBet = bettingBank?.find(([actorId]) => actorId === address)?.[1] !== 0;
      if (isHaveNoBalance && isHaveBet) {
        return { status: 'all-in' };
      }

      if (!activeParticipants?.active_ids?.includes(address)) {
        return { status: 'fold' };
      }

      if (address === turn && turn !== account?.decodedAddress && isActiveGame) {
        return { status: 'thinking' };
      }

      const isActed = betting?.acted_players?.find((actorId) => actorId === address);
      if (isActed && !investedInTheCircle) {
        return { status: 'check' };
      }

      if (investedInTheCircle) {
        const [, amount] = investedInTheCircle;
        return { status: 'bet', bet: Number(amount) };
      }
      return { status: 'waiting' };
    },
    [autoFoldPlayers, alreadyInvestedInTheCircle, bettingBank, betting, activeParticipants, account, turn],
  );

  return getPlayerStatusAndBet;
};

export { useGetPlayerStatusAndBet };
