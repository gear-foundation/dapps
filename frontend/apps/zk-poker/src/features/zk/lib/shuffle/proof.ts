import { numberToLittleEndianBytes } from './bytes';

export function cpProofToBytes(proof: {
  A: { X: bigint; Y: bigint; Z: bigint };
  B: { X: bigint; Y: bigint; Z: bigint };
  z: bigint;
}): ChaumPedersenProofBytes {
  return {
    a: [
      numberToLittleEndianBytes(proof.A.X),
      numberToLittleEndianBytes(proof.A.Y),
      numberToLittleEndianBytes(proof.A.Z),
    ],
    b: [
      numberToLittleEndianBytes(proof.B.X),
      numberToLittleEndianBytes(proof.B.Y),
      numberToLittleEndianBytes(proof.B.Z),
    ],
    z: numberToLittleEndianBytes(proof.z),
  };
}
