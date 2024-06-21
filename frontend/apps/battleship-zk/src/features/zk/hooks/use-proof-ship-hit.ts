import { getHash, getHitShips, getParsedZkData, setZkData } from '@/features/zk/utils';
import { useAccount } from '@gear-js/react-hooks';
import { ADDRESS } from '@/app/consts';
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

  const requestProofHit = async (shipsField: number[][], hit: string) => {
    const ships = getHitShips(shipsField);
    const hash = await getHash(shipsField.flat());
    const payload = { ships, hash, hit };

    const res = await fetch(`${ADDRESS.ZK_PROOF_BACKEND}/api/proof/hit`, {
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
