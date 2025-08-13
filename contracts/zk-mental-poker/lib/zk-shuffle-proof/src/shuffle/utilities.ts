import { BigNumber } from "ethers";
const Scalar = require("ffjavascript").Scalar;
// @ts-ignore
import { F1Field } from "ffjavascript";
import { randomBytes } from 'crypto';

const q = BigInt("52435875175126190479447740508185965837690552500527637822603658699938581184513"); // BLS12-381 scalar field
const F = new F1Field(q);
const a = BigInt(-5);
const d = 45022363124591815672509500913686876175488063829319466900776701791074614335719n;

const base = {
  X: BigInt("0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18"),
  Y: BigInt("0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166"),
  Z: 1n,
};

// todo
export type BabyJub = any;
export type EC = any;
export type Deck = any;

export function generateRandomScalar(numBits: number): bigint {
  const byteLength = Math.ceil(numBits / 8);
  const max = 1n << BigInt(numBits);

  while (true) {
      const buf = randomBytes(byteLength);
      let sk = BigInt("0x" + buf.toString("hex"));
      if (sk < max) return sk;
  }
}
export function scalarMul(
  F: F1Field,
  a: bigint,
  d: bigint,
  P: { X: bigint, Y: bigint, Z: bigint },
  n: bigint
): { X: bigint, Y: bigint, Z: bigint } {
  if (n === 0n) {
    return { X: 0n, Y: 1n, Z: 1n }; // Нейтральный элемент
  }
  let R = { X: 0n, Y: 1n, Z: 1n }; // Нейтральный аккумулятор
  let Q = { ...P }; // Копия P для итеративного удвоения

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
  P1: { X: bigint, Y: bigint, Z: bigint },
  P2: { X: bigint, Y: bigint, Z: bigint }
): { X: bigint, Y: bigint, Z: bigint } {
  const { X: X1, Y: Y1, Z: Z1 } = P1;
  const { X: X2, Y: Y2, Z: Z2 } = P2;

  const A = F.mul(Z1, Z2);                // A = Z1 * Z2
  const B = F.square(A);                  // B = A^2
  const C = F.mul(X1, X2);                // C = X1 * X2
  const D = F.mul(Y1, Y2);                // D = Y1 * Y2
  const E = F.mul(F.mul(d, C), D);        // E = d * C * D
  const F_ = F.sub(B, E);                 // F = B - E
  const G = F.add(B, E);                  // G = B + E

  const X1plusY1 = F.add(X1, Y1);
  const X2plusY2 = F.add(X2, Y2);
  const X1Y1_X2Y2 = F.mul(X1plusY1, X2plusY2); // (X1 + Y1)(X2 + Y2)
  const CD = F.add(C, D);                        // C + D
  const E_ = F.sub(X1Y1_X2Y2, CD);               // E = (X1 + Y1)(X2 + Y2) - (C + D)

  const X3 = F.mul(F.mul(A, F_), E_);     // X3 = A * F * E
  const Y3 = F.mul(F.mul(A, G), F.sub(D, F.mul(a, C))); // Y3 = A * G * (D - a * C)
  const Z3 = F.mul(F_, G);                // Z3 = F * G

  return { X: X3, Y: Y3, Z: Z3 };
}

/// Throws an error if `condition` is not true.
export function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message || "Assertion Failed");
  }
}

/// Samples field elements between 0 ~ min(2**numBits-1, Fr size).
export function sampleFieldElements(
  babyjub: BabyJub,
  numElements: bigint,
): bigint[] {
  const arr = [];
  let num: bigint;
  const threshold  = babyjub.subOrder; 
  for (let i = 0; i < numElements; i++) {
    do {
      num = Scalar.fromRprLE(babyjub.F.random());
    } while (Scalar.geq(num, threshold));
    arr.push(num);
  }
  return arr;
}

/// Compresses an array of boolean into a bigint.
export function bits2Num(arr: boolean[]): bigint {
  let res = 0n;
  let power = 1n;
  for (let i = 0; i < arr.length; i++) {
    res += BigInt(arr[i]) * power;
    power *= 2n;
  }
  return res;
}

/// Decomposes `num` into a boolean array of bits.
export function num2Bits(num: bigint, length: number): boolean[] {
  const bits: boolean[] = [];
  while (num > 0) {
    const tmp = Boolean(num % 2n);
    bits.push(tmp);
    num = (num - (num % 2n)) / 2n;
  }
  while (bits.length < length) {
    bits.push(false);
  }
  return bits;
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

/// Aggregates public keys into a single public key.
/// aggregateKey = \sum_{i=0}^n pks[i]
export function keyAggregate(babyJub: BabyJub, pks: EC[]): EC {
  let aggregateKey = [babyJub.F.e("0"), babyJub.F.e("1")];
  for (let i = 0; i < pks.length; i++) {
    aggregateKey = babyJub.addPoint(aggregateKey, pks[i]);
  }
  return aggregateKey;
}

/// Samples a nxn permutation matrix.
export function samplePermutation(n: number): bigint[] {
  const array = [...Array(n).keys()];
  let currentIndex = array.length - 1;
  while (currentIndex !== 0) {
    const randomIndex = Math.floor(Math.random() * currentIndex);
    [array[currentIndex], array[randomIndex]] = [array[randomIndex], array[currentIndex]];
    currentIndex--;
  }
  const matrix = Array(n * n).fill(0);
  for (let i = 0; i < n; i++) {
    matrix[i * n + array[i]] = 1;
  }
  const matrixBigint: bigint[] = [];
  for (let i = 0; i < n * n; i++) {
    matrixBigint[i] = BigInt(matrix[i]);
  }
  return matrixBigint;
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

/// Converts the type of pks to be string.
export function convertPk(babyjub: BabyJub, pks: EC[]): bigint[][] {
  const arr = [];
  for (let i = 0; i < pks.length; i++) {
    const pk: string[] = [];
    pk.push(babyjub.F.toString(pks[i][0]));
    pk.push(babyjub.F.toString(pks[i][1]));
    arr.push(string2Bigint(pk));
  }
  return arr;
}

/// Computes B = A \times X.
export function matrixMultiplication(
  A: bigint[],
  X: bigint[],
  numRows: number,
  numCols: number,
): bigint[] {
  assert(A.length === numRows * numCols, "Shape of A should be numRows x numCols");
  assert(X.length === numCols, "Length of X should be numCols");
  const B: bigint[] = [];
  for (let i = 0; i < numRows; i++) {
    let tmp = 0n;
    for (let j = 0; j < numCols; j++) {
      tmp += A[i * numCols + j] * X[j];
    }
    B.push(tmp);
  }
  return B;
}

/// Compresses an array of elliptic curve points into compressed format.
/// For each ec point (xi,yi), we have compressed format (xi, si) where si is a 1-bit selector.
/// In particular, we can find a delta_i \in {0,1,...,(q-1)/2} given xi and recover
/// yi = s_i * delta_i + (1-s_i) * (q-delta_i).
/// This function compresses an array of ec from format
///     [x1, x2, ..., xn, y1, y2, ..., yn]
/// to the compressed format
///     [x1, x2, ..., xn, s]
/// s can be bit decomposed into s1, s2, ..., sn.
/// Assumption: the length of input `ecArr` is less than 254; ec is on Baby Jubjub curve.
export function ecCompress(ecArr: bigint[]): {
  xArr: bigint[];
  deltaArr: bigint[];
  selector: bigint;
} {
  assert(ecArr.length < 254 * 2, "Length of ecArr should be less than 254*2.");
  const q = 21888242871839275222246405745257275088548364400416034343698204186575808495617n;
  const q_minus1_over2 =
    10944121435919637611123202872628637544274182200208017171849102093287904247808n;
  const deltaArr: bigint[] = [];
  const selectorArr: boolean[] = [];
  for (let i = ecArr.length / 2; i < ecArr.length; i++) {
    if (ecArr[i] <= q_minus1_over2) {
      selectorArr.push(true);
      deltaArr.push(ecArr[i]);
    } else {
      selectorArr.push(false);
      deltaArr.push(q - ecArr[i]);
    }
  }
  const selector: bigint = bits2Num(selectorArr);
  const xArr: bigint[] = [];
  for (let i = 0; i < ecArr.length / 2; i++) {
    xArr.push(ecArr[i]);
  }
  return { xArr, deltaArr, selector };
}

/// Decompresses into an array of elliptic curve points from the compressed format `xArr`, `deltaArr`, and `selector`.
export function ecDecompress(xArr: bigint[], deltaArr: bigint[], selector: bigint): bigint[] {
  assert(xArr.length < 254, "Length of xArr should be less than 254");
  assert(xArr.length === deltaArr.length, "Length of xArr should equal to the length of deltaArr");
  const selectorArr: boolean[] = num2Bits(selector, deltaArr.length);
  assert(
    selectorArr.length === deltaArr.length,
    "Length mismatch. selectorArr.length: " +
      String(selectorArr.length) +
      ", deltaArr.length: " +
      String(deltaArr.length),
  );
  const q = 21888242871839275222246405745257275088548364400416034343698204186575808495617n;
  const ecArr: bigint[] = [];
  for (let i = 0; i < xArr.length; i++) {
    ecArr.push(xArr[i]);
  }
  for (let i = 0; i < selectorArr.length; i++) {
    const flag = BigInt(selectorArr[i]);
    ecArr.push(flag * deltaArr[i] + (1n - flag) * (q - deltaArr[i]));
  }
  return ecArr;
}

/// Compresses a deck of cards with the following layout:
///     [
///         x00, x01, ..., x0{n-1},
///         y00, y01, ..., y0{n-1},
///         x10, x11, ..., x1{n-1},
///         y10, y11, ..., y1{n-1},
///     ]
export function compressDeck(deck: bigint[]): {
  X0: bigint[];
  X1: bigint[];
  delta0: bigint[];
  delta1: bigint[];
  selector: bigint[];
} {
  const deck0 = deck.slice(0, deck.length / 2);
  const deck1 = deck.slice(deck.length / 2, deck.length);
  const compressedDeck0 = ecCompress(deck0);
  const compressedDeck1 = ecCompress(deck1);
  const s: bigint[] = [];
  s.push(compressedDeck0.selector);
  s.push(compressedDeck1.selector);
  return {
    X0: compressedDeck0.xArr,
    X1: compressedDeck1.xArr,
    delta0: compressedDeck0.deltaArr,
    delta1: compressedDeck1.deltaArr,
    selector: s,
  };
}

/// Decompresses a deck of cards.
export function decompressDeck(
  X0: bigint[],
  X1: bigint[],
  Y0_delta: bigint[],
  Y1_delta: bigint[],
  s: bigint[],
): bigint[] {
  const decompressedDeck0 = ecDecompress(X0, Y0_delta, s[0]);
  const decompressedDeck1 = ecDecompress(X1, Y1_delta, s[1]);
  const deck: bigint[] = [];
  for (let i = 0; i < decompressedDeck0.length; i++) {
    deck.push(decompressedDeck0[i]);
  }
  for (let i = 0; i < decompressedDeck1.length; i++) {
    deck.push(decompressedDeck1[i]);
  }
  return deck;
}

/// Prints an array to match circom input format.
export function printArray(arr: bigint[]) {
  let str = "[";
  for (let i = 0; i < arr.length; i++) {
    // eslint-disable-next-line quotes
    str += '"' + String(arr[i]);
    if (i < arr.length - 1) {
      // eslint-disable-next-line quotes
      str += '", ';
    }
  }
  str += "],";
  return str;
}

/// Given x coordinate of a point on baby jubjub curve, returns a delta such that
///     (a * x^2 + delta^2 = 1 + d * x^2 * delta^2) % q
///     0 <= delta <= (q-1)/2
export function ecX2Delta(babyjub: BabyJub, x: bigint): bigint {
  const q = 21888242871839275222246405745257275088548364400416034343698204186575808495617n;
  const q_minus1_over2 =
    10944121435919637611123202872628637544274182200208017171849102093287904247808n;
  const xFq = babyjub.F.e(x);
  const a = babyjub.F.e(168700);
  const d = babyjub.F.e(168696);
  const one = babyjub.F.e(1);
  const xSquare = babyjub.F.square(xFq);
  let delta = babyjub.F.sqrt(
    babyjub.F.div(
      babyjub.F.sub(babyjub.F.mul(a, xSquare), one),
      babyjub.F.sub(babyjub.F.mul(d, xSquare), one),
    ),
  );
  delta = Scalar.fromRprLE(babyjub.F.fromMontgomery(delta));
  if (delta > q_minus1_over2) {
    delta = q - delta;
  }
  return delta;
}

/// Receovers an array of delta from an array of x-coordinate of points on babyjubjub curve.
export function recoverDeck(
  babyjub: BabyJub,
  X0: bigint[],
  X1: bigint[],
): { Delta0: bigint[]; Delta1: bigint[] } {
  const Delta0: bigint[] = [];
  const Delta1: bigint[] = [];
  for (let i = 0; i < X0.length; i++) {
    Delta0.push(ecX2Delta(babyjub, X0[i]));
    Delta1.push(ecX2Delta(babyjub, X1[i]));
  }
  return { Delta0, Delta1 };
}

/// Converts an array of string to an array of bigint.
export function string2Bigint(arr: string[]): bigint[] {
  const output: bigint[] = [];
  for (let i = 0; i < arr.length; i++) {
    output.push(BigInt(arr[i]));
  }
  return output;
}

/// Prepares `x0`, `x1`, `selector0`, and `selector1` queried from contract to the input data for decryption.
export function prepareDecryptData(
  babyjub: BabyJub,
  x0: BigNumber,
  x1: BigNumber,
  selector0: BigNumber,
  selector1: BigNumber,
  numCards: number,
  cardIdx: number,
): bigint[] {
  const Y = [];
  const q = 21888242871839275222246405745257275088548364400416034343698204186575808495617n;
  const delta0 = ecX2Delta(babyjub, x0.toBigInt());
  const delta1 = ecX2Delta(babyjub, x1.toBigInt());
  const flag0 = BigInt(num2Bits(selector0.toBigInt(), numCards)[cardIdx]);
  const flag1 = BigInt(num2Bits(selector1.toBigInt(), numCards)[cardIdx]);
  // Y layout: [c0.x, c0.y, c1.x, c1.y]
  Y.push(x0.toBigInt());
  Y.push(flag0 * delta0 + (1n - flag0) * (q - delta0));
  Y.push(x1.toBigInt());
  Y.push(flag1 * delta1 + (1n - flag1) * (q - delta1));
  return Y;
}

// Prepares deck queried from contract to the deck for generating ZK proof.
export function prepareShuffleDeck(
  babyjub: BabyJub,
  deck: Deck,
  numCards: number,
): { X0: bigint[]; X1: bigint[]; Selector: bigint[]; Delta: bigint[][] } {
  const deckX0: bigint[] = [];
  const deckX1: bigint[] = [];
  for (let i = 0; i < numCards; i++) {
    deckX0.push(deck.X0[i].toBigInt());
  }
  for (let i = 0; i < numCards; i++) {
    deckX1.push(deck.X1[i].toBigInt());
  }
  const deckDelta = recoverDeck(babyjub, deckX0, deckX1);
  return {
    X0: deckX0,
    X1: deckX1,
    Selector: [deck.selector0._data.toBigInt(), deck.selector1._data.toBigInt()],
    Delta: [deckDelta.Delta0, deckDelta.Delta1],
  };
}