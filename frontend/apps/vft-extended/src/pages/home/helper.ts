import { decodeAddress, encodeAddress } from '@gear-js/api';

export const isValidAddress = (address: string): boolean => {
  try {
    encodeAddress(decodeAddress(address));
    return true;
  } catch {
    return false;
  }
};

export function toActorId(address: string) {
  return decodeAddress(address);
}
