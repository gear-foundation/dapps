export interface ECPoint {
  X: bigint;
  Y: bigint;
  Z: bigint;
}

export type CipherCard = { c0: ECPoint; c1: ECPoint };

export interface EncryptedCard {
  c0: Array<`0x${string}`>;
  c1: Array<`0x${string}`>;
}

export interface ChaumPedersenProofBytes {
  a: Array<`0x${string}`>;
  b: Array<`0x${string}`>;
  z: `0x${string}`;
}

export interface ProofBytes {
  a: `0x${string}`;
  b: `0x${string}`;
  c: `0x${string}`;
}

export interface Card {
  suit: string;
  rank: string;
  point: ECPoint;
}
