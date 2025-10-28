/* eslint-disable */
import { HexString } from '@gear-js/api';

import { Card as GameCard } from '@/features/zk/api/types';
import { getRankFromValue } from '@/features/zk/utils';

import { HandRank } from '../types';
import { solvePokerHand } from './poker-hand-solver';

const getWinnersHand = (
  winners?: `0x${string}`[],
  revealedPlayers?: [HexString, [Card, Card]][],
  commonCardsFields?: (GameCard | null)[],
): { winnersHand: GameCard[]; handRank: HandRank } | null => {
  if (!winners?.length || !revealedPlayers?.length || !commonCardsFields || commonCardsFields.includes(null)) return null;

  const winnersCards: GameCard[] =
    revealedPlayers
      ?.find(([playerAddress]) => playerAddress === winners?.[0])?.[1]
      ?.map((card) => ({
        suit: card.suit,
        rank: getRankFromValue(card.value),
      })) || [];

  const allCards = [...winnersCards, ...(commonCardsFields as GameCard[])];
  const result = solvePokerHand(allCards);

  if (!result) return null;

  return { winnersHand: result.handCards, handRank: result.handRank };
};

export { getWinnersHand };
