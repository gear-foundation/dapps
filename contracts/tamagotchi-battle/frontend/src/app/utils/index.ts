import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { withoutCommas } from '@gear-js/react-hooks';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function toNumber(value: string) {
  return +withoutCommas(value);
}

export { isHexValue, hexRequired, isExists } from './form-validations';
