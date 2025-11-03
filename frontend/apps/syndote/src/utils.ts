import { decodeAddress } from '@gear-js/api';

export const getSafeDecodedAddress = (address?: string) => {
  if (address) {
    try {
      return decodeAddress(address.trim());
    } catch (_error) {
      // empty
    }
  }
  return null;
};
