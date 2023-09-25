import { signatureVerify } from '@polkadot/util-crypto';

export function isValidSig(publicKey: string, msg: string): boolean {
  try {
    const result = signatureVerify(publicKey, msg, publicKey);
    return result.isValid;
  } catch (error) {
    return false;
  }
}
