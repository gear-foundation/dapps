import { ECPoint } from '../../api/types';

import { hexToBigIntLE, numberToLittleEndianBytes } from './bytes';

export function ecPointToHexLE(point: ECPoint): Array<`0x${string}`> {
  return [numberToLittleEndianBytes(point.X), numberToLittleEndianBytes(point.Y), numberToLittleEndianBytes(point.Z)];
}

export function toECPoint(c0: `0x${string}`[]) {
  return {
    X: hexToBigIntLE(c0[0]),
    Y: hexToBigIntLE(c0[1]),
    Z: hexToBigIntLE(c0[2]),
  };
}
