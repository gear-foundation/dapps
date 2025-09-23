import {  initDeck, keyGen, scalarMul, projectiveAdd, elgamalEncryptDeck, generatePermutation, permuteMatrix} from 'zk-shuffle-proof';

import { resolve } from 'path';
import { readFileSync, writeFileSync } from 'fs';
// @ts-ignore
import { F1Field } from "ffjavascript";
// @ts-ignore
import { groth16 } from "snarkjs";

const snarkjs = require('snarkjs');

const q = BigInt("52435875175126190479447740508185965837690552500527637822603658699938581184513"); // BLS12-381 scalar field
const F = new F1Field(q);
const neutral = { X: 0n, Y: 1n, Z: 1n };
const a = BigInt(-5);
const d = 45022363124591815672509500913686876175488063829319466900776701791074614335719n;
const base = {
    X: BigInt("0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18"),
    Y: BigInt("0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166"),
    Z: 1n,
  };


type CipherCard = {
  c0: ECPoint;
  c1: ECPoint;
};
interface ECPoint {
  X: bigint;
  Y: bigint;
  Z: bigint;
}

interface Card {
  suit: string;
  rank: string;
  point: ECPoint;
}

const SUITS = ['hearts', 'diamonds', 'clubs', 'spades'];
const RANKS = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'];


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

  return cards.find(card => {
    const cardAffine = toAffine(F, card.point);
    return F.eq(cardAffine.x, targetAffine.x) && F.eq(cardAffine.y, targetAffine.y);
  });
}

async function generateDecryptProof(
  c0: ECPoint,
  sk: bigint,
  decryptWasmFile: string,
  decryptZkeyFile: string,
  decryptVkey: string
): Promise<{
  dec: ECPoint;
  proof: any;
  publicSignals: any;
  isValid: boolean;
}> {
  const skC0 = scalarMul(F, a, d, c0, sk);
  const dec: ECPoint = {
    X: F.neg(skC0.X),
    Y: skC0.Y,
    Z: skC0.Z
  };

  const input = {
    c0: [c0.X.toString(), c0.Y.toString(), c0.Z.toString()],
    sk: sk.toString(), 
    expected: [dec.X.toString(), dec.Y.toString(), dec.Z.toString()]
  };

  console.time("fullProve");
  const { proof, publicSignals } = await groth16.fullProve(
    input,
    decryptWasmFile,
    decryptZkeyFile
  );
  console.timeEnd("fullProve");

  const isValid = await snarkjs.groth16.verify(decryptVkey, publicSignals, proof);
  return { dec, proof, publicSignals, isValid };
}

function bigintToBytes48(x: string): Uint8Array {
  const hex = BigInt(x).toString(16).padStart(96, "0"); 
  return Uint8Array.from(Buffer.from(hex, "hex"));
}

function serializeG1Uncompressed([x, y, _z]: string[]): Uint8Array {
  const xBytes = bigintToBytes48(x);
  const yBytes = bigintToBytes48(y);
  return new Uint8Array([...xBytes, ...yBytes]);
}

function serializeG2Uncompressed([[x0, x1], [y0, y1], _z]: string[][]): Uint8Array {
  const x1Bytes = bigintToBytes48(x1);
  const x0Bytes = bigintToBytes48(x0);
  const y1Bytes = bigintToBytes48(y1);
  const y0Bytes = bigintToBytes48(y0);
  return new Uint8Array([...x1Bytes, ...x0Bytes, ...y1Bytes, ...y0Bytes]);
}

function encodeProof(proof: {
  pi_a: string[],
  pi_b: string[][],
  pi_c: string[],
}): {
  a: Uint8Array;
  b: Uint8Array;
  c: Uint8Array;
} {
  return {
    a: serializeG1Uncompressed(proof.pi_a),
    b: serializeG2Uncompressed(proof.pi_b),
    c: serializeG1Uncompressed(proof.pi_c),
  };
}
async function main() {
  const encryptWasmFile = resolve(__dirname, '../circuits/build/shuffle_encrypt/shuffle_encrypt_js/shuffle_encrypt.wasm');
  const encryptZkeyFile = resolve(__dirname, '../circuits/build/shuffle_encrypt/shuffle_encrypt.zkey');
  const encryptVkey = await snarkjs.zKey.exportVerificationKey(new Uint8Array(Buffer.from(readFileSync(encryptZkeyFile))));

  const decryptWasmFile = resolve(__dirname, '../circuits/build/decrypt/decrypt_js/decrypt.wasm');
  const decryptZkeyFile = resolve(__dirname, '../circuits/build/decrypt/decrypt.zkey');
  const decryptVkey = await snarkjs.zKey.exportVerificationKey(new Uint8Array(Buffer.from(readFileSync(decryptZkeyFile))));

  writeFileSync(
    'output/decrypt_vkey.json',
    JSON.stringify(decryptVkey, null, 2)
  );

  writeFileSync(
    'output/shuffle_vkey.json',
    JSON.stringify(encryptVkey, null, 2)
  );


  // CLIENT-SERVER INTERACTION FLOW FOR ZK SHUFFLE PROTOCOL
  const numCards = 52;

  const numPlayers = 6;
  const numBits = 64;
  const players = Array.from({ length: numPlayers }, () => keyGen(numBits));

  const playerPks = players.map((player, i) => ({
    index: i,
    pk: {
      X: player.pk.X.toString(),
      Y: player.pk.Y.toString(),
      Z: player.pk.Z.toString(),
    },
  }));
  
  writeFileSync('output/player_pks.json', JSON.stringify(playerPks, null, 2));
  
  const playerSks = players.map((player, i) => ({
    index: i,
    sk: player.sk.toString() 
  }));
  writeFileSync('output/player_sks.json', JSON.stringify(playerSks, null, 2));

  // Mapping cards to elliptic curve points (used for later matching)
  let deck: bigint[][] = initDeck(numCards);

  const cardMap = buildCardMap(deck);

  writeFileSync(
    'output/card_map.json',
    JSON.stringify(cardMap, (_, value) =>
      typeof value === 'bigint' ? value.toString() : value,
      2
    )
  );

  // STEP 1: BACKEND SIDE
  // After registration — backend already has public keys of players collected from contract events
  // Calculates aggregate public key = pk1 + pk2 + pk3
  const aggKey = players.reduce(
    (acc, player) => projectiveAdd(F, a, d, acc, player.pk),
    { X: 0n, Y: 1n, Z: 1n }
  );

  const allProofs: any[] = [];
  const allPublicSignals: any[] = [];

  // STEP 2: BACKEND SIDE — generate initial  deck 
  // Backend sends compressedDeck and aggKey to the first player
  
  // STEP 3: CLIENT SIDE — each player shuffles & encrypts the deck
  // Shuffle phase: each player encrypts and permutes the deck
  for (let i = 0; i < numPlayers; i++) {
    console.log(`\nPlayer ${i + 1} shuffling...`);

    // Client generates shuffle permutation and randomness
    const permutation = generatePermutation(numCards);

    const { encrypted, rScalars } = elgamalEncryptDeck(F, a, d, base, aggKey, deck);
    const R = rScalars.map(r => r.toString());

    const shuffled = permuteMatrix(encrypted, permutation);
    
    const input = {
      pk: [aggKey.X.toString(), aggKey.Y.toString(), aggKey.Z.toString()],
      R,
      original: deck.map((row) => row.map((v) => v.toString())),
      permuted: shuffled.map((row) => row.map((v) => v.toString())),
    };
    console.time("fullProve");
    // Client generates SNARK proof of correct shuffle
    const { proof, publicSignals } = await groth16.fullProve(
      input,
      encryptWasmFile,
      encryptZkeyFile
    );
    console.timeEnd("fullProve");

    // Client sends result to backend (deck + proof)
    // Payload example:
    // {
    //   gameId: string,
    //   playerPk: [string, string],
    //   proof: {
    //     pi_a: string[],
    //     pi_b: string[][],
    //     pi_c: string[]
    //   },
    //   publicSignals: string[],
    //   encryptedDeck: bigint[][]
    // }
    // This payload is used by the backend to:
    // - verify SNARK proof
    // - update game state with newly encrypted deck
    // - broadcast deck to next player
   
    // Backend verifies the proof
    const isValid = await snarkjs.groth16.verify(encryptVkey, publicSignals,proof);

    if (!isValid) {
      throw new Error(`Invalid shuffle proof from Player ${i + 1}`);
    }
    allProofs.push(proof);
    allPublicSignals.push(publicSignals);

    // Backend updates deck to latest encrypted state
    deck = shuffled;
   }

   const shuffleProofsData = allProofs.map((proof, i) => ({
    proof,
    publicSignals: allPublicSignals[i],
  }));
  
  writeFileSync('output/shuffle_proofs.json', JSON.stringify(shuffleProofsData, null, 2));
  const encryptedDeck = deck.map(row => row.map(v => v.toString()));

  writeFileSync('output/encrypted_deck.json', JSON.stringify(encryptedDeck, null, 2));
  // STEP 4: CONTRACT SIDE — decompress deck and distribute cards

  // Distribute 2 cards to each player
  const playerHands: CipherCard[][] = [];
  
  for (let i = 0; i < numPlayers; i++) {
    const hand: CipherCard[] = [];
  
    for (let j = 0; j < 2; j++) { 
      const cardIndex = i * 2 + j;
  
      const c0: ECPoint = {
      X: deck[0][cardIndex],
      Y: deck[1][cardIndex],
      Z: deck[2][cardIndex],
    };

    const c1: ECPoint = {
      X: deck[3][cardIndex],
      Y: deck[4][cardIndex],
      Z: deck[5][cardIndex],
    };
  
      hand.push({ c0, c1 });
    }
  
    playerHands.push(hand);
  }
  

  // Contract emits event with assigned encrypted cards to players

  // STEP 5: BACKEND SIDE — prepare decryption assignments

  const K = playerHands[0].length; 

  const partialDecryptProofs = [];
  const partialSumDecs = Array(numPlayers).fill(null).map(() => Array(K).fill(neutral));

  for (let i = 0; i < numPlayers; i++) { // Игрок i — decryptor
    for (let j = 0; j < numPlayers; j++) {
      if (i === j) continue; // Не расшифровываем свои карты

      for (let k = 0; k < K; k++) {
        const c0 = playerHands[j][k].c0;

        const {
          dec,
          proof,
          publicSignals,
          isValid
        } = await generateDecryptProof(
          c0,
          players[i].sk,
          decryptWasmFile,
          decryptZkeyFile,
          decryptVkey
        );

        if (!isValid) {
          throw new Error(`Invalid proof by player ${i} for card ${k} of player ${j}`);
        }

        partialDecryptProofs.push({
          proof,
          publicSignals
        });

        partialSumDecs[j][k] = projectiveAdd(F, a, d, partialSumDecs[j][k], dec);
      }
    }
  }
  

  writeFileSync(
    'output/partial_decrypt_proofs.json',
    JSON.stringify(partialDecryptProofs, null, 2)
  );
  // CLIENT SIDE
  const partiallyDecCardsList = [];

  for (let i = 0; i < numPlayers; i++) {
    const partiallyDecCards = playerHands[i].map((card, k) =>
      projectiveAdd(F, a, d, card.c1, partialSumDecs[i][k])
    );
  
    partiallyDecCardsList.push({
      publicKey: {
        X: players[i].pk.X.toString(),
        Y: players[i].pk.Y.toString(),
        Z: players[i].pk.Z.toString(),
      },
      cards: playerHands[i].map((card, k) => ({
        c0: {
          X: card.c0.X.toString(),
          Y: card.c0.Y.toString(),
          Z: card.c0.Z.toString(),
        },
        c1_partial: {
          X: partiallyDecCards[k].X.toString(),
          Y: partiallyDecCards[k].Y.toString(),
          Z: partiallyDecCards[k].Z.toString(),
        }
      }))
    });
  }
  
  const playerDecryptions = [];

  for (let i = 0; i < players.length; i++) {
    const decryptedCardsWithProofs = [];
    const sk = players[i].sk;
    const pk = players[i].pk;
    const hand = playerHands[i];
    const c1_partials = partiallyDecCardsList[i].cards.map(card => ({
      X: BigInt(card.c1_partial.X),
      Y: BigInt(card.c1_partial.Y),
      Z: BigInt(card.c1_partial.Z)
    }));

    for (let k = 0; k < 2; k ++) {
      const c0 = hand[k].c0;
      const {
        dec,
        proof,
        publicSignals,
        isValid
      } = await generateDecryptProof(
        c0,
        sk,
        decryptWasmFile,
        decryptZkeyFile,
        decryptVkey
      );
      let decCard = projectiveAdd(F, a, d, c1_partials[k], dec);
      const match = findCardByPoint(F, cardMap, decCard);
      const label = `Player ${i + 1} card ${k + 1}`;
      if (match) {
        console.log(`${label}: 🃏 ${match.rank} of ${match.suit}`);
      } else {
        console.log(`${label}: ❓ Unknown card`);
      }
      decryptedCardsWithProofs.push({
        decrypted: {
          X: decCard.X.toString(),
          Y: decCard.Y.toString(),
          Z: decCard.Z.toString()
        },
        proof,
        publicSignals
      });
    }
    console.log("amount of cards", decryptedCardsWithProofs.length)
    playerDecryptions.push({
      publicKey: {
        X: pk.X.toString(),
        Y: pk.Y.toString(),
        Z: pk.Z.toString()
      },
      cards: decryptedCardsWithProofs
    });
  }
  
  writeFileSync(
    "output/player_decryptions.json",
    JSON.stringify(playerDecryptions, null, 2)
  );

    const usedCards = numPlayers * 2;

    const tableCards: CipherCard[] = [];

    for (let i = 0; i < 5; i++) {
      const cardIndex = usedCards + i;

      const c0: ECPoint = {
        X: deck[0][cardIndex],
        Y: deck[1][cardIndex],
        Z: deck[2][cardIndex],
      };

      const c1: ECPoint = {
        X: deck[3][cardIndex],
        Y: deck[4][cardIndex],
        Z: deck[5][cardIndex],
      };

      tableCards.push({ c0, c1 });
    }

    const playerDecryptionsData = [];

    const partialSumDecsTable: ECPoint[] = [];
    for (let i = 0; i < 5; i++) {
      partialSumDecsTable.push(neutral);
    }
    for (let playerIndex = 0; playerIndex < numPlayers; playerIndex++) {
      const sk = players[playerIndex].sk;
      const pk = players[playerIndex].pk;

      const decryptions = [];

      for (let cardIndex = 0; cardIndex < 5; cardIndex++) {
        const card = tableCards[cardIndex];
        const c0 = card.c0;
    
        const {
          dec,
          proof,
          publicSignals,
          isValid
        } = await generateDecryptProof(
          c0,
          sk,
          decryptWasmFile,
          decryptZkeyFile,
          decryptVkey
        );
    
        if (!isValid) {
          throw new Error(`Invalid decryption proof for card ${cardIndex} by player ${playerIndex}`);
        }
    
        partialSumDecsTable[cardIndex] = projectiveAdd(
          F, a, d,
          partialSumDecsTable[cardIndex],
          dec
        );
    
        decryptions.push({
          encryptedCard: {
            c0: {
              X: card.c0.X.toString(),
              Y: card.c0.Y.toString(),
              Z: card.c0.Z.toString()
            },
            c1: {
              X: card.c1.X.toString(),
              Y: card.c1.Y.toString(),
              Z: card.c1.Z.toString()
            }
          },
          dec: {
            X: dec.X.toString(),
            Y: dec.Y.toString(),
            Z: dec.Z.toString()
          },
          proof,
          publicSignals
        });
      }
      playerDecryptionsData.push({
        playerPubKey: {
          X: pk.X.toString(),
          Y: pk.Y.toString(),
          Z: pk.Z.toString()
        },
        decryptions
      });
    }
    writeFileSync("output/table_decryptions.json", JSON.stringify(playerDecryptionsData, null, 2));

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