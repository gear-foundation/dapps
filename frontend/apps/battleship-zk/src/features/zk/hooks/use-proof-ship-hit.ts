import { useAccount } from '@gear-js/react-hooks';

import { ENV } from '@/app/consts';
import { getArrangementShips, getHash, getParsedZkData, setZkData } from '@/features/zk/utils';

import { GameType, ZkProofData } from '../types';

export const useProofShipHit = () => {
  const { account } = useAccount();

  const clearProofData = (gameType: GameType) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    if (!zkData?.[gameType]?.['proof-data']) {
      return;
    }

    delete zkData[gameType]?.['proof-data'];

    setZkData(account, zkData);
  };

  const saveProofData = (gameType: GameType, proofData: ZkProofData) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, [gameType]: { ...zkData[gameType], [`proof-data`]: proofData } }
      : {
          [gameType]: {
            [`proof-data`]: proofData,
          },
        };

    setZkData(account, newZkData);
  };

  const getProofData = (gameType: GameType) => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.[gameType]?.['proof-data'] || null;
  };

  const requestProofHit = async (shipsField: number[][], hit: string, hits: string[]) => {
    const ships = getArrangementShips(shipsField);

    const hash = await getHash(shipsField.flat());
    const payload = { ...ships, hash, hit, hits };

    try {
      const res = await fetch(`${ENV.ZK_PROOF_BACKEND}/api/proof/hit`, {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
          'Content-Type': 'application/json',
        },
      });
      const proofData = await res.json();

      return proofData;
    } catch (_error) {
      throw new Error('Failed to fetch proof data');
    }
  };

  return { requestProofHit, getProofData, saveProofData, clearProofData };
};
