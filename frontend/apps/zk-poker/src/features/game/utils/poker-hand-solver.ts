/* eslint-disable */
// @ts-expect-error
import { Hand } from 'pokersolver';

import { Card as GameCard, Rank } from '@/features/zk/api/types';
import { SUITS } from '@/features/zk/utils/consts';

import { HandRank } from '../types';

export const toPokersolverCard = (card: GameCard) => {
  const rank = card.rank === '10' ? 'T' : card.rank;
  const suit = card.suit[0].toLowerCase();
  return `${rank}${suit}`;
};

export const fromPokersolverCard = (card: { value: string; suit: string }) => {
  const { value, suit } = card;
  const rank = (value === 'T' ? '10' : value) as Rank;

  return {
    suit: SUITS.find((original) => original.startsWith(suit.toUpperCase())) as Suit,
    rank,
  };
};

/**
 * Solves poker hand from game cards and returns hand rank and best hand cards
 */
export const solvePokerHand = (
  cards: GameCard[],
): { handRank: HandRank; handCards: GameCard[] } | null => {
  if (cards.length < 5) {
    return null;
  }

  try {
    const pokersolverCards = cards.map(toPokersolverCard);
    const hand = Hand.solve(pokersolverCards);
    const handRank = hand.name.replace('-', ' ') as HandRank;
    const handCards = hand.cards.map(fromPokersolverCard);
    return { handRank, handCards };
  } catch (error) {
    console.error('Error solving poker hand:', error);
    return null;
  }
};

export const getCurrentHandRank = (
  playerCards?: [GameCard, GameCard] | null,
  commonCardsFields?: (GameCard | null)[],
): HandRank | null => {
  if (!playerCards || !commonCardsFields || commonCardsFields.every((card) => card === null)) {
    return null;
  }

  const revealedCommonCards = commonCardsFields.filter((card): card is GameCard => card !== null);

  if (revealedCommonCards.length < 3) {
    return null;
  }

  const allCards = [...playerCards, ...revealedCommonCards];
  const result = solvePokerHand(allCards);
  
  return result?.handRank || null;
};

