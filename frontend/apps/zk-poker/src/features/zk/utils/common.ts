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

export { hexToBigIntLE };
