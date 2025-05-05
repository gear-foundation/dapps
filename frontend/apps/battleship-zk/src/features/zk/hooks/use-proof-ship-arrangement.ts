import { ADDRESS } from '@/app/consts';
import { getArrangementShips, getHash } from '@/features/zk/utils';

import { ZkProofData } from '../types';

export const useProofShipArrangement = () => {
  const requestZKProof = async (shipsField: number[][]): Promise<ZkProofData> => {
    const ships = getArrangementShips(shipsField);
    const hash = await getHash(shipsField.flat());
    const payload = { ...ships, hash };

    try {
      const res = await fetch(`${ADDRESS.ZK_PROOF_BACKEND}/api/proof/placement`, {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
        'Content-Type': 'application/json',
      },
    });
    const proofData = await res.json();

      return proofData;
    } catch (error) {
      throw new Error('Failed to fetch proof data');
    }
  };

  return { requestZKProof };
};
