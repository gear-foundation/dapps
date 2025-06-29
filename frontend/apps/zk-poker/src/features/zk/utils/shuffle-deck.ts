import { groth16 } from 'snarkjs';

import { ZkTaskShuffle } from '../api/types';
import { elgamalEncryptDeck, generatePermutation, permuteMatrix } from '../lib';

import { curveParams } from './consts';

const encryptWasmFile = '/shuffle_encrypt.wasm';
const encryptZkeyFile = '/shuffle_encrypt.zkey';

const shuffleDeck = async (zkTaskShuffle: ZkTaskShuffle) => {
  const { deck, aggKey } = zkTaskShuffle;

  const pk = {
    X: BigInt(aggKey.X),
    Y: BigInt(aggKey.Y),
    Z: BigInt(aggKey.Z),
  };
  const bigintDeck = deck.map((row) => row.map((v) => BigInt(v)));

  const { a, d, base, F } = curveParams;
  const { encrypted, rScalars } = elgamalEncryptDeck(F, a, d, base, pk, bigintDeck);

  const numCards = deck[0].length;

  // shuffle deck
  const permutation = generatePermutation(numCards);
  const shuffled = permuteMatrix(encrypted, permutation);

  const getFullProof = async () => {
    const input = {
      pk: [aggKey.X.toString(), aggKey.Y.toString(), aggKey.Z.toString()],
      R: rScalars.map((r) => r.toString()),
      original: bigintDeck.map((row) => row.map((v) => v.toString())),
      permuted: shuffled.map((row) => row.map((v) => v.toString())),
    };

    const { proof, publicSignals } = await groth16.fullProve(input, encryptWasmFile, encryptZkeyFile, undefined, {
      memorySize: 128,
    });

    return { proof, publicSignals };
  };

  const { proof, publicSignals } = await getFullProof();
  const { pi_a, pi_b, pi_c } = proof;

  const result = {
    proof: { pi_a, pi_b, pi_c },
    publicSignals,
    deck: shuffled.map((row) => row.map((v) => v.toString())),
  };

  const isProofValid = publicSignals[0] === '1';
  if (!isProofValid) throw new Error('Proof is not valid.');

  return result;
};

export { shuffleDeck };
