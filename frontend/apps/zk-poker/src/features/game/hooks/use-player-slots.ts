import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useMemo } from 'react';

import { Card, PlayerStatus } from '@/features/zk/api/types';
import { getRankFromValue } from '@/features/zk/utils';

import { useParticipantsQuery, useRevealedPlayersQuery } from '../sails';

import { useGameStatus } from './use-game-status';
import { useGetPlayerStatusAndBet } from './use-get-player-status-and-bet';
import { usePlayerCards } from './use-player-cards';

export interface PlayerSlot {
  address: HexString;
  name: string;
  chips: number;
  cards: [Card, Card] | null | undefined;
  isMe: boolean;
  status: PlayerStatus;
  bet?: number;
}

export const usePlayerSlots = (
  turn: HexString | null,
  autoFoldPlayers: HexString[],
  dillerAddress?: HexString,
): PlayerSlot[] => {
  const { account } = useAccount();

  const { isActiveGame, isGameStarted, pots } = useGameStatus();
  const { participants } = useParticipantsQuery();
  const { revealedPlayers } = useRevealedPlayersQuery({ enabled: isActiveGame });
  const { playerCards } = usePlayerCards(isGameStarted) || {};
  const getPlayerStatusAndBet = useGetPlayerStatusAndBet(turn, autoFoldPlayers);

  const getPlayerCards = useMemo(() => {
    return (address: string): [Card, Card] | null | undefined => {
      if (address === account?.decodedAddress && playerCards) {
        return playerCards as [Card, Card];
      }

      const revealedPlayer = revealedPlayers?.find(([playerAddress]) => playerAddress === address);
      if (revealedPlayer) {
        return revealedPlayer[1].map((card) => ({
          suit: card.suit,
          rank: getRankFromValue(card.value),
        })) as [Card, Card];
      }

      return playerCards ? null : undefined;
    };
  }, [account?.decodedAddress, playerCards, revealedPlayers]);

  return useMemo(() => {
    return (
      participants?.map(([address, participant]) => ({
        address,
        name: participant.name,
        chips: Number(participant.balance),
        cards: getPlayerCards(address),
        isMe: address === account?.decodedAddress,
        ...getPlayerStatusAndBet(address, participant, isActiveGame, pots),
        isDiller: address === dillerAddress,
      })) || []
    );
  }, [participants, getPlayerCards, account?.decodedAddress, getPlayerStatusAndBet, isActiveGame, pots, dillerAddress]);
};
