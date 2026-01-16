import { ProofBytes, ChaumPedersenProofBytes } from "../types.js";
import { numberToLittleEndianBytes, toHexString, bigintToBytes48 } from "./bytes.js";

export function cpProofToBytes(proof: {
  A: { X: bigint; Y: bigint; Z: bigint };
  B: { X: bigint; Y: bigint; Z: bigint };
  z: bigint;
}): ChaumPedersenProofBytes {
  return {
    a: [numberToLittleEndianBytes(proof.A.X), numberToLittleEndianBytes(proof.A.Y), numberToLittleEndianBytes(proof.A.Z)],
    b: [numberToLittleEndianBytes(proof.B.X), numberToLittleEndianBytes(proof.B.Y), numberToLittleEndianBytes(proof.B.Z)],
    z: numberToLittleEndianBytes(proof.z),
  };
}

export function encodeProof(proof: {
  pi_a: string[];
  pi_b: string[][];
  pi_c: string[];
}): ProofBytes {
  return {
    a: toHexString(serializeG1Uncompressed(proof.pi_a)),
    b: toHexString(serializeG2Uncompressed(proof.pi_b)),
    c: toHexString(serializeG1Uncompressed(proof.pi_c)),
  };
}

function serializeG1Uncompressed([x, y, _z]: string[]): Uint8Array {
  const xBytes = bigintToBytes48(x);
  const yBytes = bigintToBytes48(y);
  return new Uint8Array([...xBytes, ...yBytes]);
}

function serializeG2Uncompressed([
  [x0, x1],
  [y0, y1],
  _z,
]: string[][]): Uint8Array {
  const x1Bytes = bigintToBytes48(x1);
  const x0Bytes = bigintToBytes48(x0);
  const y1Bytes = bigintToBytes48(y1);
  const y0Bytes = bigintToBytes48(y0);
  return new Uint8Array([...x1Bytes, ...x0Bytes, ...y1Bytes, ...y0Bytes]);
}

export function publicSignalsToBytes(publicSignals: string[]): Array<`0x${string}`> {
  const BYTES = 32;
  const out: `0x${string}`[] = [];
  for (const s of publicSignals) {
    const v = BigInt(s);
    const arr = new Uint8Array(BYTES);
    for (let i=0;i<BYTES;i++) arr[i] = Number((v >> BigInt(8*i)) & 0xffn);
    out.push(("0x" + [...arr].map(b=>b.toString(16).padStart(2,"0")).join("")) as `0x${string}`);
  }
  return out;
}