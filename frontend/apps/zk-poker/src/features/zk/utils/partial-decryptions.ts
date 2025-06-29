import { groth16 } from 'snarkjs';

import { DecryptOtherPlayersCardsResult, ECPoint, OtherPlayersCard } from '../api/types';
import { scalarMul } from '../lib/shuffle/utilities';

import { curveParams, decryptWasmFilePath, decryptZkeyFilePath } from './consts';

// ! TODO: remove duplicates in other files
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

export function toHexString(bytes: Uint8Array) {
  const hexString = [...bytes].map((b) => b.toString(16).padStart(2, '0')).join('');

  return ('0x' + hexString) as `0x${string}`;
}

function bigintToBytes48(x: string): Uint8Array {
  const hex = BigInt(x).toString(16).padStart(96, '0');
  return Uint8Array.from(Buffer.from(hex, 'hex'));
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

function encodeProof(proof: { pi_a: string[]; pi_b: string[][]; pi_c: string[] }): ProofBytes {
  return {
    a: toHexString(serializeG1Uncompressed(proof.pi_a)),
    b: toHexString(serializeG2Uncompressed(proof.pi_b)),
    c: toHexString(serializeG1Uncompressed(proof.pi_c)),
  };
}

// Convert public signals (bigint array) to bytes in little endian format
function publicSignalsToBytes(publicSignals: string[]): Array<`0x${string}`> {
  const BYTES_PER_SIGNAL = 32;

  const result: `0x${string}`[] = [];
  for (const sig of publicSignals) {
    const out: `${string}`[] = [];
    // parse into a BigInt (handles both "0x…" and decimal strings)
    const v = BigInt(sig);

    // extract each byte (little-endian)
    for (let i = 0; i < BYTES_PER_SIGNAL; i++) {
      const byte = Number((v >> BigInt(8 * i)) & BigInt(0xff));
      out.push(`${byte.toString(16).padStart(2, '0')}`);
    }
    result.push(('0x' + out.join('')) as `0x${string}`);
  }

  return result;
}

const partialDecryptions = async (c0: ECPoint<string>, sk: bigint) => {
  const { a, d, F } = curveParams;
  const bigintC0 = { X: BigInt(c0.X), Y: BigInt(c0.Y), Z: BigInt(c0.Z) };
  const skC0 = scalarMul(F, a, d, bigintC0, sk);

  const dec: ECPoint = { X: F.neg(skC0.X), Y: skC0.Y, Z: skC0.Z };

  const input = {
    c0: [c0.X, c0.Y, c0.Z],
    sk: sk.toString(),
    expected: [dec.X.toString(), dec.Y.toString(), dec.Z.toString()],
  };

  const { proof, publicSignals } = await groth16.fullProve(input, decryptWasmFilePath, decryptZkeyFilePath, undefined, {
    memorySize: 128,
  });
  const { pi_a, pi_b, pi_c } = proof;

  return { proof: { pi_a, pi_b, pi_c }, publicSignals, decryptedC0: dec };
};

const partialDecryptionsForPlayersCards = async (encryptedCards: OtherPlayersCard[], sk: bigint) => {
  const results = [];

  for (const cardEntry of encryptedCards) {
    const { playerIndex, cardIndex, c0 } = cardEntry;

    const { proof, publicSignals } = await partialDecryptions(c0, sk);

    results.push({ playerIndex, cardIndex, c0, proof, publicSignals });
  }

  return results as DecryptOtherPlayersCardsResult[];
};

const partialDecryptionsForTableCards = async (
  encryptedCards: EncryptedCard[],
  sk: bigint,
): Promise<{
  instances: Array<VerificationVariables>;
}> => {
  const instances: Array<VerificationVariables> = [];

  for (const encryptedCard of encryptedCards) {
    const { c0 } = encryptedCard;

    const c0AsItIs2 = {
      X: hexToBigIntLE(c0[0]).toString(),
      Y: hexToBigIntLE(c0[1]).toString(),
      Z: hexToBigIntLE(c0[2]).toString(),
    };
    const { proof, publicSignals } = await partialDecryptions(c0AsItIs2, sk);

    const endodedProof = encodeProof(proof);

    instances.push({
      proof_bytes: endodedProof,
      public_input: publicSignalsToBytes(publicSignals),
    });
  }

  return { instances };
};

export {
  partialDecryptions,
  partialDecryptionsForPlayersCards,
  partialDecryptionsForTableCards,
  publicSignalsToBytes,
  encodeProof,
};
