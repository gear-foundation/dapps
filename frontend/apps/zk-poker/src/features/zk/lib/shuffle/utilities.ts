/* eslint-disable */
// @ts-ignore
import { Scalar, F1Field, F1FieldInstance } from 'ffjavascript';
import { blake2b } from '@noble/hashes/blake2.js';
import { randomBytes } from 'crypto';
import { curveParams } from '../../utils/consts';

const { a, d, base, scalarField, F, FQ_BYTES, FR_BYTES } = curveParams;

// todo
export type BabyJub = any;
export type EC = any;
export type Deck = any;

function toFixedLE(x: bigint, len: number): Uint8Array {
  let hex = x.toString(16);
  if (hex.length % 2) hex = '0' + hex;
  const be = Uint8Array.from(Buffer.from(hex, 'hex'));
  const le = new Uint8Array(len);
  const n = Math.min(len, be.length);
  for (let i = 0; i < n; i++) le[i] = be[be.length - 1 - i];
  return le;
}

function concatBytes(chunks: Uint8Array[]): Uint8Array {
  const total = chunks.reduce((s, c) => s + c.length, 0);
  const out = new Uint8Array(total);
  let off = 0;
  for (const c of chunks) {
    out.set(c, off);
    off += c.length;
  }
  return out;
}
export function generateRandomScalar(numBits: number): bigint {
  const byteLength = Math.ceil(numBits / 8);
  const max = 1n << BigInt(numBits);

  while (true) {
    const buf = randomBytes(byteLength);
    let sk = BigInt('0x' + buf.toString('hex'));
    if (sk < max) return sk;
  }
}

function bytesToBigIntLE(a: Uint8Array): bigint {
  let x = 0n,
    mul = 1n;
  for (const b of a) {
    x += BigInt(b) * mul;
    mul <<= 8n;
  }
  return x;
}

export function hashToFr(F: F1FieldInstance, points: { X: bigint; Y: bigint; Z: bigint }[]): bigint {
  const chunks: Uint8Array[] = [];

  for (const P of points) {
    const zInv = F.inv(P.Z);
    const x = F.mul(P.X, zInv);
    const y = F.mul(P.Y, zInv);
    chunks.push(toFixedLE(x, FQ_BYTES));
    chunks.push(toFixedLE(y, FQ_BYTES));
  }

  const digest = blake2b(concatBytes(chunks), { dkLen: 64 });
  const c = bytesToBigIntLE(digest.slice(0, FR_BYTES)) % scalarField;
  return c;
}

export function cpProve(
  F: F1FieldInstance,
  a: bigint,
  d: bigint,
  g: { X: bigint; Y: bigint; Z: bigint },
  pk: { X: bigint; Y: bigint; Z: bigint },
  c0: { X: bigint; Y: bigint; Z: bigint },
  dec: { X: bigint; Y: bigint; Z: bigint },
  sk: bigint,
): { A: { X: bigint; Y: bigint; Z: bigint }; B: { X: bigint; Y: bigint; Z: bigint }; z: bigint } {
  const r = generateRandomScalar(50);

  const A = scalarMul(F, a, d, g, r); // r·g
  const B = scalarMul(F, a, d, c0, r); // r·c1

  const c = hashToFr(F, [g, pk, c0, dec, A, B]);
  const z = (r + c * sk) % scalarField;
  return { A, B, z };
}

export function cpVerify(
  F: any,
  a: bigint,
  d: bigint,
  g: { X: bigint; Y: bigint; Z: bigint },
  pk: { X: bigint; Y: bigint; Z: bigint },
  c0: { X: bigint; Y: bigint; Z: bigint },
  dec: { X: bigint; Y: bigint; Z: bigint },
  proof: { A: { X: bigint; Y: bigint; Z: bigint }; B: { X: bigint; Y: bigint; Z: bigint }; z: bigint },
): boolean {
  const c = hashToFr(F, [g, pk, c0, dec, proof.A, proof.B]);
  const lhs1 = scalarMul(F, a, d, g, proof.z);
  const rhs1 = projectiveAdd(F, a, d, proof.A, scalarMul(F, a, d, pk, c));

  const lhs2 = scalarMul(F, a, d, c0, proof.z);
  const rhs2 = projectiveAdd(F, a, d, proof.B, scalarMul(F, a, d, dec, c));

  return eqProj(F, lhs1, rhs1) && eqProj(F, lhs2, rhs2);
}

function eqProj(F: any, P: { X: bigint; Y: bigint; Z: bigint }, Q: { X: bigint; Y: bigint; Z: bigint }): boolean {
  return F.mul(P.X, Q.Z) === F.mul(Q.X, P.Z) && F.mul(P.Y, Q.Z) === F.mul(Q.Y, P.Z);
}

export function scalarMul(
  F: F1FieldInstance,
  a: bigint,
  d: bigint,
  P: { X: bigint; Y: bigint; Z: bigint },
  n: bigint,
): { X: bigint; Y: bigint; Z: bigint } {
  if (n === 0n) {
    return { X: 0n, Y: 1n, Z: 1n };
  }
  let R = { X: 0n, Y: 1n, Z: 1n };
  let Q = { ...P };

  while (n > 0n) {
    if (n & 1n) {
      R = projectiveAdd(F, a, d, R, Q);
    }
    Q = projectiveAdd(F, a, d, Q, Q);
    n >>= 1n;
  }

  return R;
}

export function projectiveAdd(
  F: any,
  a: bigint,
  d: bigint,
  P1: { X: bigint; Y: bigint; Z: bigint },
  P2: { X: bigint; Y: bigint; Z: bigint },
): { X: bigint; Y: bigint; Z: bigint } {
  const { X: X1, Y: Y1, Z: Z1 } = P1;
  const { X: X2, Y: Y2, Z: Z2 } = P2;

  const A = F.mul(Z1, Z2); // A = Z1 * Z2
  const B = F.square(A); // B = A^2
  const C = F.mul(X1, X2); // C = X1 * X2
  const D = F.mul(Y1, Y2); // D = Y1 * Y2
  const E = F.mul(F.mul(d, C), D); // E = d * C * D
  const F_ = F.sub(B, E); // F = B - E
  const G = F.add(B, E); // G = B + E

  const X1plusY1 = F.add(X1, Y1);
  const X2plusY2 = F.add(X2, Y2);
  const X1Y1_X2Y2 = F.mul(X1plusY1, X2plusY2); // (X1 + Y1)(X2 + Y2)
  const CD = F.add(C, D); // C + D
  const E_ = F.sub(X1Y1_X2Y2, CD); // E = (X1 + Y1)(X2 + Y2) - (C + D)

  const X3 = F.mul(F.mul(A, F_), E_); // X3 = A * F * E
  const Y3 = F.mul(F.mul(A, G), F.sub(D, F.mul(a, C))); // Y3 = A * G * (D - a * C)
  const Z3 = F.mul(F_, G); // Z3 = F * G

  return { X: X3, Y: Y3, Z: Z3 };
}

/// Throws an error if `condition` is not true.
export function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message || 'Assertion Failed');
  }
}

// Generates a secret key between 0 ~ min(2**numBits-1, Fr size).
export function keyGen(numBits: number): {
  sk: bigint;
  pk: { X: bigint; Y: bigint; Z: bigint };
} {
  const sk = generateRandomScalar(numBits);
  const pk = scalarMul(F, a, d, base, sk);
  return {
    sk,
    pk,
  };
}

/// Initializes a deck of `numCards` cards. Each card is represented as 2 elliptic curve
/// points (c0i.x, c0i.y,c0i.z, c1i.x, c1i.y, c1i.z)
/// Layout: [
///     c01.x, ..., c0n.x,
///     c01.y, ..., c0n.y,
///     c01.z, ..., c0n.z,
///     c11.x, ..., c1n.x,
///     c11.y, ..., c1n.y,
///     c11.z, ..., c1n.z,
/// ]
export function initDeck(numCards: number): bigint[][] {
  const deck: bigint[][] = Array.from({ length: 6 }, () => Array(numCards).fill(0n));

  for (let i = 0; i < numCards; i++) {
    const scalar = BigInt(i + 1);
    deck[0][i] = 0n;
    deck[1][i] = 1n;
    deck[2][i] = 1n;
    const P = scalarMul(F, a, d, base, scalar);
    deck[3][i] = P.X;
    deck[4][i] = P.Y;
    deck[5][i] = P.Z;
  }

  return deck;
}

/// Searches the deck for a card. If the card is in the deck, returns the card index.
/// If the card is not in the deck, return -1.
export function searchDeck(deck: bigint[], cardX1: bigint, numCards: number): number {
  for (let i = 0; i < numCards; i++) {
    if (deck[2 * numCards + i] === cardX1) {
      return i;
    }
  }
  return -1;
}
