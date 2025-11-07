import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export * from './sails';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const prettifyText = (text: string) => {
  return text.slice(0, 6) + '...' + text.slice(-4);
};
