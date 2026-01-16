import { numberToLittleEndianBytes, bigintToBytes32LEArray } from "./bytes.js";
export function ecPointToHexLE(point) {
    return [
        numberToLittleEndianBytes(point.X),
        numberToLittleEndianBytes(point.Y),
        numberToLittleEndianBytes(point.Z),
    ];
}
export function ecPointToNumberArrays(point) {
    return {
        x: bigintToBytes32LEArray(point.X),
        y: bigintToBytes32LEArray(point.Y),
        z: bigintToBytes32LEArray(point.Z),
    };
}
export function ecPointToBytes(point) {
    return [
        numberToLittleEndianBytes(point.X),
        numberToLittleEndianBytes(point.Y),
        numberToLittleEndianBytes(point.Z),
    ];
}
export function toAffine(F, P) {
    return { x: F.div(P.X, P.Z), y: F.div(P.Y, P.Z) };
}
