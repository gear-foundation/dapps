import { Account } from '@gear-js/react-hooks';

import { VerificationVariables } from '@/app/utils/sails/lib/lib';
import { buildPoseidon } from '@/features/zk/utils/poseidon';

import { GameType, ZkData, ZkProofData } from '../types';

type PoseidonInstance = {
  (inputs: Array<number | string>): unknown;
  F: {
    toString: (value: unknown) => string;
  };
};

const isPoseidonInstance = (value: unknown): value is PoseidonInstance =>
  typeof value === 'function' && typeof (value as { F?: { toString?: unknown } }).F?.toString === 'function';

export const getHash = async (data: Array<number | string>): Promise<string> => {
  const poseidonResult: unknown = await buildPoseidon();

  if (!isPoseidonInstance(poseidonResult)) {
    throw new Error('Poseidon instance is not available');
  }

  const hash = poseidonResult(data);

  return poseidonResult.F.toString(hash);
};

export const getArrangementShips = (shipsField: number[][]) =>
  shipsField.reduce(
    (acc, shipCells, index) => ({
      ...acc,
      [`ship_${index + 1}`]: shipCells.map((cell) => cell.toString()),
    }),
    {} as Record<string, string[]>,
  );

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

export const isZkProofData = (value: unknown): value is ZkProofData => {
  if (!value || typeof value !== 'object') {
    return false;
  }

  const maybeProof = value as Partial<ZkProofData>;
  const { proofContent, publicContent } = maybeProof;

  if (!proofContent || typeof proofContent !== 'object') {
    return false;
  }

  if (!publicContent || typeof publicContent !== 'object') {
    return false;
  }

  const { results } = publicContent;

  return Array.isArray(results);
};
