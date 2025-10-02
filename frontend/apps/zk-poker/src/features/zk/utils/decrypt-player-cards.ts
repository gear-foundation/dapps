import { F1FieldInstance } from 'ffjavascript';

import { Card, CardWithPoint, ECPoint, Rank } from '../api/types';
import { projectiveAdd, scalarMul, initDeck } from '../lib';
import { toECPoint } from '../lib/shuffle/ec';

import { curveParams, RANKS, SUITS } from './consts';
import { partialDecryption } from './partial-decryptions';

function toAffine(F: F1FieldInstance, P: ECPoint) {
  const x = F.div(P.X, P.Z);
  const y = F.div(P.Y, P.Z);
  return { x, y };
}

function findCardByPoint(F: F1FieldInstance, cards: CardWithPoint[], target: ECPoint): Card | undefined {
  const targetAffine = toAffine(F, target);

  return cards.find((card) => {
    const cardAffine = toAffine(F, card.point);
    return F.eq(cardAffine.x, targetAffine.x) && F.eq(cardAffine.y, targetAffine.y);
  });
}

function buildCardMap(deck: bigint[][]): CardWithPoint[] {
  const numCards = SUITS.length * RANKS.length;
  if (deck[0].length !== numCards) {
    throw new Error(`Deck size mismatch: expected ${numCards}, got ${deck[0].length}`);
  }

  const cards: CardWithPoint[] = [];

  for (let s = 0; s < SUITS.length; s++) {
    for (let r = 0; r < RANKS.length; r++) {
      const i = s * RANKS.length + r;
      cards.push({
        suit: SUITS[s] as Suit,
        rank: RANKS[r] as Rank,
        point: {
          X: deck[3][i],
          Y: deck[4][i],
          Z: deck[5][i],
        },
      });
    }
  }

  return cards;
}

const numCards = 52;
const deck = initDeck(numCards);
const cardMap = buildCardMap(deck);

export type DecryptedCardsResult = {
  cards: Card[];
  myCardsC0: ECPoint[];
};

const decryptCards = async (encryptedCards: EncryptedCard[], sk: bigint): Promise<DecryptedCardsResult> => {
  const { F, a, d } = curveParams;

  const result = await Promise.all(
    encryptedCards.map((card) => {
      const c0 = toECPoint(card.c0);
      const c1 = toECPoint(card.c1);
      const skC0 = scalarMul(F, a, d, c0, sk);
      const dec: ECPoint = {
        X: F.neg(skC0.X),
        Y: skC0.Y,
        Z: skC0.Z,
      };

      const decryptedPoint = projectiveAdd(F, a, d, c1, dec);
      const match = findCardByPoint(F, cardMap, decryptedPoint);

      if (match) {
        return {
          card: { suit: match.suit, rank: match.rank },
          c0,
        };
      } else {
        throw new Error('Unknown card');
      }
    }),
  );

  return {
    cards: result.map(({ card }) => card),
    myCardsC0: result.map(({ c0 }) => c0),
  };
};

const getMyDecryptedCardsProof = (myCardsC0: ECPoint[], sk: bigint, pk: ECPoint): PartialDec[] => {
  return myCardsC0.map((c0) => partialDecryption(c0, sk, pk));
};

export { decryptCards, getMyDecryptedCardsProof };
