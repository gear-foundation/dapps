import { getArrangementShips, getHash } from '@/features/zk/utils';

export const useProofShipArrangement = () => {
  const requestZKProof = async (shipsField: number[][]) => {
    const ships = getArrangementShips(shipsField);
    const hash = await getHash(shipsField.flat());
    const payload = { ...ships, hash };

    const res = await fetch('https://stg-zk-proof.vara.network/api/proof/placement', {
      method: 'POST',
      body: JSON.stringify(payload),
      headers: {
        'Content-Type': 'application/json',
      },
    });
    const proofData = await res.json();

    return proofData;
  };

  return { requestZKProof };
};
