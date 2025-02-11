import { AlertContainerFactory } from '@gear-js/react-hooks';
import { isHex } from '@polkadot/util';
import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

import { DominoNumber, DominoTileType, StateDominoNumber, StateDominoTileType } from '../types/game';

export const cx = (...styles: string[]) => clsx(...styles);

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const copyToClipboard = async (key: string, alert: AlertContainerFactory, successfulText?: string) => {
  try {
    await navigator.clipboard.writeText(key);
    alert.success(successfulText || 'Copied');
  } catch (err) {
    alert.error('Copy error');
  }
};

export const getBgColors = (v: number) => {
  switch (v) {
    case 0:
      return { train: 'text-[#EB5757]', backdrop: 'bg-[#EB5757]', isLight: false, isDark: false };
    case 1:
      return { train: 'text-[#0075A7]', backdrop: 'bg-[#0075A7]', isLight: false, isDark: true };
    case 2:
      return { train: 'text-[#F2C34C]', backdrop: 'bg-[#F2C34C]', isLight: false, isDark: false };
    case 3:
      return { train: 'text-[#F0FE51]', backdrop: 'bg-[#F0FE51]', isLight: true, isDark: false };
    case 4:
      return { train: 'text-[#353535]', backdrop: 'bg-[#353535]', isLight: false, isDark: true };
    case 5:
      return { train: 'text-[#219653]', backdrop: 'bg-[#219653]', isLight: false, isDark: true };
    case 6:
      return { train: 'text-[#00D1FF]', backdrop: 'bg-[#00D1FF]', isLight: false, isDark: false };
    case 7:
      return { train: 'text-[#670E9D]', backdrop: 'bg-[#670E9D]', isLight: false, isDark: true };
    default:
      return { train: 'text-[#EBF1EE]', backdrop: 'bg-[#67CB4D]', isLight: false, isDark: false };
  }
};

const strings = [
  'Zero',
  'One',
  'Two',
  'Three',
  'Four',
  'Five',
  'Six',
  'Seven',
  'Eight',
  'Nine',
  'Ten',
  'Eleven',
  'Twelve',
];

export const getTileId = (tile: DominoTileType, tiles: StateDominoTileType[]): number => {
  const objFromArray = { left: strings[+tile[0]], right: strings[+tile[1]] };
  return tiles.findIndex((tile) => tile.left === objFromArray.left && tile.right === objFromArray.right);
};

export const isSubset = (array1: any[], array2: any[]) => array2.every((element) => array1.includes(element));
export const isPartialSubset = (array1: any[], array2: any[]) => array2.some((element) => array1.includes(element));

export const hexRequired = (value: string) =>
  !value ? 'Field is required' : !isHex(value) ? 'String must be in Hex format' : null;
export const stringRequired = (value: string) => (!value ? 'Field is required' : null);
export const numberRequired = (value: number | null | undefined) =>
  value === null || value === undefined ? 'Field is required' : null;

const stringToNumberMapping: Record<StateDominoNumber, DominoNumber> = {
  Zero: '0',
  One: '1',
  Two: '2',
  Three: '3',
  Four: '4',
  Five: '5',
  Six: '6',
  Seven: '7',
  Eight: '8',
  Nine: '9',
  Ten: '10',
  Eleven: '11',
  Twelve: '12',
};

const convertTileStringToNumbers = (tile: StateDominoTileType): DominoTileType => {
  return [stringToNumberMapping[tile.left], stringToNumberMapping[tile.right]];
};

export const findTile = (startTileString: string, tiles: StateDominoTileType[]): DominoTileType | null => {
  const index = parseInt(startTileString, 10);
  if (index >= 0 && index < tiles.length) {
    return convertTileStringToNumbers(tiles[index]);
  }
  return null;
};

// function to convert tile in specific format to [number, number]
export const convertFormattedTileToNumbers = (formattedTile: StateDominoTileType) => {
  return convertTileStringToNumbers(formattedTile);
};

export const shortenString = (str: string, length: number): string => `${str.slice(0, length)}...${str.slice(-length)}`;
