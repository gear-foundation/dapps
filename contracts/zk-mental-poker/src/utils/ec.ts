import { ECPoint } from "../types.js";
import { numberToLittleEndianBytes, bigintToBytes32LEArray } from "./bytes.js";

export function ecPointToHexLE(point: ECPoint): Array<`0x${string}`> {
  return [
    numberToLittleEndianBytes(point.X),
    numberToLittleEndianBytes(point.Y),
    numberToLittleEndianBytes(point.Z),
  ];
}

export function ecPointToNumberArrays(point: ECPoint): { x:number[]; y:number[]; z:number[] } {
  return {
    x: bigintToBytes32LEArray(point.X),
    y: bigintToBytes32LEArray(point.Y),
    z: bigintToBytes32LEArray(point.Z),
  };
}

export function ecPointToBytes(point: ECPoint): Array<`0x${string}`> {
  return [
    numberToLittleEndianBytes(point.X),
    numberToLittleEndianBytes(point.Y),
    numberToLittleEndianBytes(point.Z),
  ];
}

export function toAffine(F: any, P: ECPoint) {
  return { x: F.div(P.X, P.Z), y: F.div(P.Y, P.Z) };
}
