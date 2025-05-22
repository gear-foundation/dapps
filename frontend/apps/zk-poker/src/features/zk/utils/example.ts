/* eslint-disable */

// @ts-ignore
import { F1Field } from 'ffjavascript';
import { groth16 } from 'snarkjs';
import {
  initDeck,
  keyGen,
  scalarMul,
  projectiveAdd,
  elgamalEncryptDeck,
  generatePermutation,
  permuteMatrix,
} from '../lib';

// import encryptWasmFile from './shuffle_encrypt.wasm';
// import encryptZkeyFile from './shuffle_encrypt.zkey';
// import decryptWasmFile from './decrypt.wasm';
// import decryptZkeyFile from './decrypt.zkey';

// ! TODO: check it
const encryptWasmFile = './shuffle_encrypt.wasm';
const encryptZkeyFile = './shuffle_encrypt.zkey';
const decryptWasmFile = './decrypt.wasm';
const decryptZkeyFile = './decrypt.zkey';

const q = BigInt('52435875175126190479447740508185965837690552500527637822603658699938581184513'); // BLS12-381 scalar field
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const F = new F1Field(q);
const a = BigInt(-5);
const d = 45022363124591815672509500913686876175488063829319466900776701791074614335719n;
const base = {
  X: BigInt('0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18'),
  Y: BigInt('0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166'),
  Z: 1n,
};

function bigintToBytes32BE(x: bigint): Uint8Array {
  const hex = BigInt(x).toString(16).padStart(64, '0');
  return Uint8Array.from(Buffer.from(hex, 'hex'));
}

type CipherCard = {
  c0: ECPoint;
  c1: ECPoint;
};
interface ECPoint {
  X: bigint;
  Y: bigint;
  Z: bigint;
}

const SUITS = ['hearts', 'diamonds', 'clubs', 'spades'];
const RANKS = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'];

interface Card {
  suit: string;
  rank: string;
  point: ECPoint;
}

function buildCardMap(deck: bigint[][]): Card[] {
  const numCards = SUITS.length * RANKS.length;
  if (deck[0].length !== numCards) {
    throw new Error(`Deck size mismatch: expected ${numCards}, got ${deck[0].length}`);
  }

  const cards: Card[] = [];

  for (let s = 0; s < SUITS.length; s++) {
    for (let r = 0; r < RANKS.length; r++) {
      const i = s * RANKS.length + r;
      cards.push({
        suit: SUITS[s],
        rank: RANKS[r],
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

function toAffine(F: any, P: ECPoint) {
  const x = F.div(P.X, P.Z);
  const y = F.div(P.Y, P.Z);
  return { x, y };
}

function findCardByPoint(F: any, cards: Card[], target: ECPoint): Card | undefined {
  const targetAffine = toAffine(F, target);

  return cards.find((card) => {
    const cardAffine = toAffine(F, card.point);
    return F.eq(cardAffine.x, targetAffine.x) && F.eq(cardAffine.y, targetAffine.y);
  });
}

const numBits = 64;
const numCards = 52;
const { sk, pk } = keyGen(numBits);

async function main() {
  // convert pk for smart contract using bigintToBytes32BE

  // Client Request: GET /get-task?lobby=0xAddress&player=0xAddress
  // Server response (if it's the player's turn):
  // {
  //      "aggregatedPublicKey": { X: String, Y: String: Z: String }
  //      "deck": bigint[][]

  // For example:
  const aggKey = pk;
  const deck = initDeck(numCards);
  const cardMap = buildCardMap(deck);
  // encrypt deck
  const { encrypted, rScalars } = elgamalEncryptDeck(F, a, d, base, aggKey, deck);
  const R = rScalars.map((r) => r.toString());

  // shuffle deck
  const permutation = generatePermutation(numCards);
  const shuffled = permuteMatrix(encrypted, permutation);

  const getFullProof = async () => {
    // prove
    const input = {
      pk: [aggKey.X.toString(), aggKey.Y.toString(), aggKey.Z.toString()],
      R,
      original: deck.map((row) => row.map((v) => v.toString())),
      permuted: shuffled.map((row) => row.map((v) => v.toString())),
    };

    const { proof, publicSignals } = await groth16.fullProve(input, encryptWasmFile, encryptZkeyFile);
    console.log('🚀 ~ publicSignals:', publicSignals);
    console.log('🚀 ~ proof:', proof);
    return { proof, publicSignals };
  };

  // send to server { proof, publicSignals }

  // after succesfull shuffle contract distributes cards to players
  // here is the contract action:
  const numPlayers = 3;
  const playerHands: CipherCard[][] = [];

  for (let i = 0; i < numPlayers; i++) {
    const hand: CipherCard[] = [];

    for (let j = 0; j < 2; j++) {
      const cardIndex = i * 2 + j;

      const c0: ECPoint = {
        X: shuffled[0][cardIndex],
        Y: shuffled[1][cardIndex],
        Z: shuffled[2][cardIndex],
      };

      const c1: ECPoint = {
        X: shuffled[3][cardIndex],
        Y: shuffled[4][cardIndex],
        Z: shuffled[5][cardIndex],
      };

      hand.push({ c0, c1 });
    }

    playerHands.push(hand);
  }
  // the next task is to decrypt players cards
  // for example client receives:
  const encryptedCards = [
    {
      cardOwner: '0xPlayer2Address',
      cardIndex: 0,
      card: playerHands[1][0],
    },
    {
      cardOwner: '0xPlayer2Address',
      cardIndex: 1,
      card: playerHands[1][1],
    },
    {
      cardOwner: '0xPlayer3Address',
      cardIndex: 0,
      card: playerHands[2][0],
    },
    {
      cardOwner: '0xPlayer3Address',
      cardIndex: 1,
      card: playerHands[2][1],
    },
  ];

  // client makes partail descryption
  const results = [];

  for (const cardEntry of encryptedCards) {
    const { cardOwner, cardIndex, card } = cardEntry;
    const c0 = card.c0;
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
    const { proof, publicSignals } = await groth16.fullProve(input, decryptWasmFile, decryptZkeyFile);
    results.push({
      cardOwner,
      cardIndex,
      dec: {
        X: dec.X.toString(),
        Y: dec.Y.toString(),
        Z: dec.Z.toString(),
      },
      proof,
      publicSignals,
    });
  }
  // results must be sent to backend
  console.log(JSON.stringify(results, null, 2));

  // read cards from contract
  // for example
  const cards = playerHands[0];
  for (let i = 0; i < cards.length; i++) {
    const card = cards[i];
    const c0 = card.c0;
    const c1 = card.c1;
    const skC0 = scalarMul(F, a, d, c0, sk);
    const dec: ECPoint = {
      X: F.neg(skC0.X),
      Y: skC0.Y,
      Z: skC0.Z,
    };
    const decryptedPoint = projectiveAdd(F, a, d, c1, dec);
    const match = findCardByPoint(F, cardMap, decryptedPoint);

    if (match) {
      console.log(`${match.rank} of ${match.suit}`);
    } else {
      console.log(`Unknown card`);
    }
  }
}

main()
  .then(() => {
    console.log('Finished successfully.');
    process.exit(0);
  })
  .catch((err) => {
    console.error('Error:', err);
    process.exit(1);
  });
