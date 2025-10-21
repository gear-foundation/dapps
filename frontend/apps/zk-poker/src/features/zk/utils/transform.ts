import { ECPoint, Rank } from '../api/types';
import { numberToLittleEndianBytesArray } from '../lib/shuffle/bytes';

const getPkBytes = (pk: ECPoint) => ({
  x: numberToLittleEndianBytesArray(pk.X),
  y: numberToLittleEndianBytesArray(pk.Y),
  z: numberToLittleEndianBytesArray(pk.Z),
});

const getRankFromValue = (value: number): Rank => {
  const ranks: Rank[] = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'];
  return ranks[value - 2];
};

export { getPkBytes, getRankFromValue };
