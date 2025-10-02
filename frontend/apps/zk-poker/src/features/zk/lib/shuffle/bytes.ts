function numberToLittleEndianBytesArray(value: string | number | bigint, byteLength = 32): Uint8Array {
  const bigintValue = BigInt(value);
  const bytes = new Uint8Array(byteLength);

  for (let i = 0; i < byteLength; i++) {
    bytes[i] = Number((bigintValue >> BigInt(8 * i)) & BigInt(0xff));
  }

  return bytes;
}

function numberToLittleEndianBytes(value: string | number | bigint, byteLength = 32): `0x${string}` {
  const bytes = numberToLittleEndianBytesArray(value, byteLength);
  return ('0x' + [...bytes].map((b) => b.toString(16).padStart(2, '0')).join('')) as `0x${string}`;
}

function bytesToBigIntLE(bytes: Uint8Array): bigint {
  let result = 0n;
  for (let i = 0; i < bytes.length; i++) {
    result += BigInt(bytes[i]) << (8n * BigInt(i));
  }
  return result;
}

function hexToBigIntLE(hex: string): bigint {
  hex = hex.startsWith('0x') ? hex.slice(2) : hex;
  const bytes = hex.match(/.{1,2}/g)?.map((b) => parseInt(b, 16)) || [];
  return bytesToBigIntLE(new Uint8Array(bytes));
}

export { hexToBigIntLE, numberToLittleEndianBytes, numberToLittleEndianBytesArray };
