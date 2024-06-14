import { getHash, getHitShips, getParsedZkData, setZkData } from '@/features/zk/utils';
import { useAccount } from '@gear-js/react-hooks';
import { ZkProofData } from '../types';

export const useProofShipHit = () => {
  const { account } = useAccount();

  const clearProofData = () => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    if (!zkData?.single['proof-data']) {
      return;
    }

    delete zkData.single['proof-data'];

    setZkData(account, zkData);
  };

  const saveProofData = (proofData: ZkProofData) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, single: { ...zkData.single, [`proof-data`]: proofData } }
      : {
          single: {
            [`proof-data`]: proofData,
          },
        };

    setZkData(account, newZkData);
  };

  const getProofData = () => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.single?.['proof-data'] || null;
  };

  const requestProofHit = async (shipsField: number[][], hit: string) => {
    const ships = getHitShips(shipsField);
    const hash = await getHash(shipsField.flat());
    const payload = { ships, hash, hit };

    const res = await fetch('https://stg-zk-proof.vara.network/api/proof/hit', {
      method: 'POST',
      body: JSON.stringify(payload),
      headers: {
        'Content-Type': 'application/json',
      },
    });
    const proofData = await res.json();

    return proofData;
  };

  return { requestProofHit, getProofData, saveProofData, clearProofData };
};
