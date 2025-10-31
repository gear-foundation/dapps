import { ENV } from '@/app/consts';
import { getArrangementShips, getHash, isZkProofData } from '@/features/zk/utils';

import { ZkProofData } from '../types';

export const useProofShipArrangement = () => {
  const requestZKProof = async (shipsField: number[][]): Promise<ZkProofData> => {
    const ships = getArrangementShips(shipsField);
    const hash = await getHash(shipsField.flat());
    const payload = { ...ships, hash };

    try {
      const response = await fetch(`${ENV.ZK_PROOF_BACKEND}/api/proof/placement`, {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
          'Content-Type': 'application/json',
        },
      });
      if (!response.ok) {
        throw new Error('Failed to fetch proof data');
      }
      const proofData: unknown = await response.json();

      if (!isZkProofData(proofData)) {
        throw new Error('Received invalid proof data shape');
      }

      return proofData;
    } catch (error) {
      if (error instanceof Error) {
        throw error;
      }

      throw new Error('Failed to fetch proof data');
    }
  };

  return { requestZKProof };
};
