/* eslint-disable */
import { HexString } from '@gear-js/api';
// @ts-expect-error
import { Hand } from 'pokersolver';

import { Card as GameCard, HandRank, Rank } from '@/features/zk/api/types';
import { getRankFromValue } from '@/features/zk/utils';
import { SUITS } from '@/features/zk/utils/consts';

const toPokersolverCard = (card: GameCard) => {
  const rank = card.rank === '10' ? 'T' : card.rank;
  const suit = card.suit[0].toLowerCase();
  return `${rank}${suit}`;
};

const fromPokersolverCard = (card: { value: string; suit: string }) => {
  const { value, suit } = card;
  const rank = (value === 'T' ? '10' : value) as Rank;

  return {
    suit: SUITS.find((original) => original.startsWith(suit.toUpperCase())) as Suit,
    rank,
  };
};

const getWinnersHand = (
  winners?: `0x${string}`[],
  revealedPlayers?: [HexString, [Card, Card]][],
  commonCardsFields?: (GameCard | null)[],
): { winnersHand: GameCard[]; handRank: HandRank } | null => {
  if (!winners?.length || !revealedPlayers || !commonCardsFields || commonCardsFields.includes(null)) return null;

  const winnersCards: GameCard[] =
    revealedPlayers
      ?.find(([playerAddress]) => playerAddress === winners?.[0])?.[1]
      ?.map((card) => ({
        suit: card.suit,
        rank: getRankFromValue(card.value),
      })) || [];

  const sevenCards = [...winnersCards, ...(commonCardsFields as GameCard[])].map(toPokersolverCard);
  const hand = Hand.solve(sevenCards);
  const handRank = hand.name.replace('-', ' ');
  const winnersHand = hand.cards.map(fromPokersolverCard);
  return { winnersHand, handRank };
};

export { getWinnersHand };
