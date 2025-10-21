import { ECPoint } from '../api/types';
import { ecPointToHexLE, toECPoint } from '../lib/shuffle/ec';
import { cpProofToBytes } from '../lib/shuffle/proof';
import { cpProve, cpVerify, scalarMul } from '../lib/shuffle/utilities';

import { curveParams } from './consts';

const partialDecryption = (c0: ECPoint, sk: bigint, pk: ECPoint): PartialDec => {
  const { a, d, F, base } = curveParams;
  const skC0 = scalarMul(F, a, d, c0, sk);

  const delta: ECPoint = { X: F.neg(skC0.X), Y: skC0.Y, Z: skC0.Z };

  const proof = cpProve(F, a, d, base, pk, c0, skC0, sk);

  if (!cpVerify(F, a, d, base, pk, c0, skC0, proof)) throw new Error('Invalid CP proof');

  return { c0: ecPointToHexLE(c0), delta_c0: ecPointToHexLE(delta), proof: cpProofToBytes(proof) };
};

const partialDecryptions = (encryptedCards: EncryptedCard[], sk: bigint, pk: ECPoint): PartialDec[] => {
  const partialDecs: PartialDec[] = encryptedCards.map((encryptedCard) => {
    const c0 = toECPoint(encryptedCard.c0);
    return partialDecryption(c0, sk, pk);
  });

  return partialDecs;
};

export { partialDecryption, partialDecryptions };
