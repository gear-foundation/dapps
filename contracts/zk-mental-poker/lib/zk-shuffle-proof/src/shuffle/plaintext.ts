import { scalarMul, projectiveAdd, generateRandomScalar } from "./utilities";

// todo
export type BabyJub = any;
export type EC = any;


/// Computes ElGamal Encryption.
export function elgamalEncrypt(F: any, a: bigint, d: bigint, G: any, pk: any, msg: any) {
  const r = generateRandomScalar(50);
  const rG = scalarMul(F, a, d, G, r);
  const rPK = scalarMul(F, a, d, pk, r);
  const c0 = projectiveAdd(F, a, d, rG, { ...msg.ic0 });
  const c1 = projectiveAdd(F, a, d, rPK, { ...msg.ic1 });
  return { c0, c1, r };
}


/// Computes ElGamal Decryption.
export function elgamalDecrypt(babyJub: BabyJub, c0: EC, c1: EC, sk: bigint): EC {
  // Scalar Field Size of Baby JubJub curve
  const r = 2736030358979909402780800718157159386076813972158567259200215660948447373041n;
  // c1 - sk * c0
  return babyJub.addPoint(c1, babyJub.mulPointEscalar(c0, r - sk));
}

export function generatePermutation(n: number): number[] {
  const arr = Array.from({ length: n }, (_, i) => i);
  for (let i = n - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [arr[i], arr[j]] = [arr[j], arr[i]];
  }
  return arr;
}

export function permuteMatrix(matrix: bigint[][], permutation: number[]): bigint[][] {
  const permuted = Array.from({ length: 6 }, () => Array(52));
  for (let row = 0; row < 6; row++) {
    for (let col = 0; col < 52; col++) {
      permuted[row][col] = matrix[row][permutation[col]];
    }
  }
  return permuted;
}

export function elgamalEncryptDeck(
  F: any,
  a: bigint,
  d: bigint,
  G: any,
  pk: any,
  deck: bigint[][],
): { encrypted: bigint[][]; rScalars: bigint[] } {
  const encrypted = Array.from({ length: 6 }, () => Array(52).fill(0n));
  const rScalars: bigint[] = [];

  for (let i = 0; i < 52; i++) {
    const ic0 = {
      X: deck[0][i],
      Y: deck[1][i],
      Z: deck[2][i],
    };
    const ic1 = {
      X: deck[3][i],
      Y: deck[4][i],
      Z: deck[5][i],
    };

    const { c0, c1, r } = elgamalEncrypt(F, a, d, G, pk, { ic0, ic1 });
    rScalars.push(r);

    encrypted[0][i] = c0.X;
    encrypted[1][i] = c0.Y;
    encrypted[2][i] = c0.Z;
    encrypted[3][i] = c1.X;
    encrypted[4][i] = c1.Y;
    encrypted[5][i] = c1.Z;
  }

  return { encrypted, rScalars };
}