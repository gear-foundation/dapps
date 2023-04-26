import { AlertContainerFactory } from '@gear-js/react-hooks';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'app/consts';
import { DominoTileType, StateDominoTileType } from '../types/game';
import { isHex } from '@polkadot/util';
import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

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
export const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

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
  const objFromArray = { left: strings[tile[0]], right: strings[tile[1]] };
  return tiles.findIndex((tile) => tile.left === objFromArray.left && tile.right === objFromArray.right);
};

export const isSubset = (array1: any[], array2: any[]) => array2.every((element) => array1.includes(element));
export const isPartialSubset = (array1: any[], array2: any[]) => array2.some((element) => array1.includes(element));

export const hexRequired = (value: string) =>
  !value ? 'Field is required' : !isHex(value) ? 'String must be in Hex format' : null;
export const stringRequired = (value: string) => (!value ? 'Field is required' : null);
