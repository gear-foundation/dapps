import { buildPoseidon } from '@/features/zk/utils/poseidon';
import { Account } from '@gear-js/react-hooks';
import { GameType, ZkData } from '../types';

export const getHash = async (data: number[] | string[]) => {
  const poseidon = await buildPoseidon();
  const hash = poseidon(data);

  return poseidon.F.toString(hash);
};

export const getArrangementShips = (shipsField: number[][]) =>
  shipsField
    .sort((a, b) => (a.length > b.length ? 1 : -1))
    .reduce((acc, item, i) => ({ ...acc, [`ship_${i + 1}`]: item.map((i) => i.toString()) }), {});

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

  localStorage.removeItem(`zk-data-${account.address}`);
};
