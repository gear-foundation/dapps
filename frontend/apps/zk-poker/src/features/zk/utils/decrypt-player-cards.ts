import { F1FieldInstance } from 'ffjavascript';
import { groth16 } from 'snarkjs';

import { Card, CardWithPoint, ContractCard, ECPoint, Input, Rank, Suit } from '../api/types';
import { projectiveAdd, scalarMul, initDeck } from '../lib';

import { curveParams, decryptWasmFilePath, decryptZkeyFilePath, RANKS, SUITS } from './consts';
import { encodeProof, publicSignalsToBytes } from './partial-decryptions';
import { getValueFromRank } from './transform';

function bytesToBigIntLE(bytes: Uint8Array): bigint {
  let result = 0n;
  for (let i = 0; i < bytes.length; i++) {
    result += BigInt(bytes[i]) << (8n * BigInt(i));
  }
  return result;
}

function hexToBigIntLE(hex: string): bigint {
  hex = hex.startsWith('0x') ? hex.slice(2) : hex;
  const bytes = hex.match(/.{1,2}/g)?.map((b) => parseInt(b, 16)) || [];
  return bytesToBigIntLE(new Uint8Array(bytes));
}

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

type Instances = [ContractCard, VerificationVariables];

export type DecryptedCardsResult = {
  cards: Card[];
  inputs: Input[];
};

const decryptCards = async (encryptedCards: EncryptedCard[], sk: bigint): Promise<DecryptedCardsResult> => {
  const { F, a, d } = curveParams;

  const result = await Promise.all(
    encryptedCards.map((card) => {
      const c0 = {
        X: hexToBigIntLE(card.c0[0]),
        Y: hexToBigIntLE(card.c0[1]),
        Z: hexToBigIntLE(card.c0[2]),
      };
      const c1 = { X: hexToBigIntLE(card.c1[0]), Y: hexToBigIntLE(card.c1[1]), Z: hexToBigIntLE(card.c1[2]) };
      const skC0 = scalarMul(F, a, d, c0, sk);
      const dec: ECPoint = {
        X: F.neg(skC0.X),
        Y: skC0.Y,
        Z: skC0.Z,
      };

      const input = {
        c0: [c0.X.toString(), c0.Y.toString(), c0.Z.toString()],
        sk: sk.toString(),
        expected: [dec.X.toString(), dec.Y.toString(), dec.Z.toString()],
      };

      const decryptedPoint = projectiveAdd(F, a, d, c1, dec);
      const match = findCardByPoint(F, cardMap, decryptedPoint);

      if (match) {
        return {
          card: { suit: match.suit, rank: match.rank },
          input,
        };
      } else {
        throw new Error('Unknown card');
      }
    }),
  );

  return {
    cards: result.map(({ card }) => card),
    inputs: result.map(({ input }) => input),
  };
};

const getDecryptedCardsProof = async (inputs: Input[], cards: Card[]) => {
  const instances = await Promise.all(
    inputs.map(async (input, index) => {
      const card = cards[index];
      const { proof, publicSignals } = await groth16.fullProve(
        input,
        decryptWasmFilePath,
        decryptZkeyFilePath,
        undefined,
        { memorySize: 128 },
      );

      const contractCard: ContractCard = {
        value: getValueFromRank(card.rank),
        suit: card.suit,
      };

      const fullProof: VerificationVariables = {
        proof_bytes: encodeProof(proof),
        public_input: publicSignalsToBytes(publicSignals),
      };

      return [contractCard, fullProof] as Instances;
    }),
  );

  return { instances };
};

export { decryptCards, getDecryptedCardsProof };
