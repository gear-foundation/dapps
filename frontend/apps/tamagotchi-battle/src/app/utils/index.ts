import { withoutCommas } from '@gear-js/react-hooks';
import { u64 } from '@polkadot/types';
import { formatNumber } from '@polkadot/util';
import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function toNumber(value: string) {
  return +withoutCommas(value);
}

export const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
  navigator.userAgent,
);

export const gasLimitToNumber = (limit: u64 | undefined) => Number(withoutCommas(formatNumber(limit)));

export { isHexValue, hexRequired, isExists } from './form-validations';
