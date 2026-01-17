export function numberToLittleEndianBytes(value, byteLength = 32) {
    const bigintValue = BigInt(value);
    const bytes = new Uint8Array(byteLength);
    for (let i = 0; i < byteLength; i++) {
        bytes[i] = Number((bigintValue >> BigInt(8 * i)) & BigInt(0xff));
    }
    return ("0x" +
        [...bytes].map((b) => b.toString(16).padStart(2, "0")).join(""));
}
export function littleEndianHexToBigInt(hex) {
    const hexStr = hex.slice(2);
    const bytes = hexStr.match(/.{1,2}/g)?.map((b) => parseInt(b, 16)) ?? [];
    const beHex = bytes.reverse().map((b) => b.toString(16).padStart(2, "0")).join("");
    return BigInt("0x" + beHex);
}
export function toHexString(bytes) {
    return ("0x" + [...bytes].map((b) => b.toString(16).padStart(2, "0")).join(""));
}
export function bigintToBytes48(x) {
    const hex = BigInt(x).toString(16).padStart(96, '0');
    return Uint8Array.from(Buffer.from(hex, 'hex'));
}
export function bigintToBytes32LEArray(x) {
    const hex = x.toString(16).padStart(64, "0");
    const u = Uint8Array.from(Buffer.from(hex, "hex")).reverse();
    return Array.from(u);
}
