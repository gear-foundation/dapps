import { Rank } from '../api/types';

const hexToDecString = (hex: string) => {
  return BigInt(hex).toString();
};

const bigintToBytes32LE = (x: bigint): Uint8Array => {
  const bytes = Uint8Array.from(Buffer.from(x.toString(16).padStart(64, '0'), 'hex'));
  return new Uint8Array([...bytes].reverse());
};

const getPkBytes = (pk: { x: bigint; y: bigint; z: bigint }) => ({
  x: bigintToBytes32LE(pk.x),
  y: bigintToBytes32LE(pk.y),
  z: bigintToBytes32LE(pk.z),
});

const getRankFromValue = (value: number): Rank => {
  const ranks: Rank[] = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'];
  return ranks[value - 2];
};

const getValueFromRank = (rank: Rank) => {
  if (rank === 'A') return 14;
  if (rank === 'K') return 13;
  if (rank === 'Q') return 12;
  if (rank === 'J') return 11;
  return parseInt(rank);
};

export { hexToDecString, bigintToBytes32LE, getPkBytes, getRankFromValue, getValueFromRank };
