import { Account } from '@gear-js/react-hooks';

import { VerificationVariables } from '@/app/utils/sails/lib/lib';
import { buildPoseidon } from '@/features/zk/utils/poseidon';

import { GameType, ZkData, ZkProofData } from '../types';

export const getHash = async (data: number[] | string[]) => {
  const poseidon = await buildPoseidon();
  const hash = poseidon(data);

  return poseidon.F.toString(hash);
};

export const getArrangementShips = (shipsField: number[][]) =>
  shipsField.reduce((acc, item, i) => ({ ...acc, [`ship_${i + 1}`]: item.map((i) => i.toString()) }), {});

export const getHitShips = (shipsField: number[][]) => shipsField.flat().map((item) => item.toString());

export const getParsedZkData = (account: Account) => {
  const zkData = localStorage.getItem(`zk-data-${account.address}`);

  return zkData ? (JSON.parse(zkData) as ZkData) : null;
};

export const setZkData = (account: Account, newZkData: ZkData) => {
  localStorage.setItem(`zk-data-${account.address}`, JSON.stringify(newZkData));
};

export const clearZkData = (gameType: GameType, account: Account) => {
  const zkData = getParsedZkData(account);

  if (!zkData) {
    return;
  }

  delete zkData[gameType];

  localStorage.setItem(`zk-data-${account.address}`, JSON.stringify(zkData));
};

export const getVerificationVariables = (
  proofDataHit: ZkProofData | null | undefined,
): VerificationVariables | null => {
  if (!proofDataHit) {
    return null;
  }

  const { proofContent, publicContent } = proofDataHit;

  return {
    proof_bytes: proofContent,
    public_input: {
      hash: publicContent.publicHash,
      out: publicContent.results[0][0],
      hit: publicContent.results[1][0],
    },
  };
};
