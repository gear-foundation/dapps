import { u8aToHex } from '@polkadot/util';
import { decodeAddress } from '@polkadot/util-crypto';

export function toActorId(address?: string) {
  try {
    return u8aToHex(decodeAddress(address));
  } catch {
    return '0x0000000000000000000000000000000000000000';
  }
}