import { buildPoseidon } from '@/features/zk/utils/poseidon';

export const getHash = async (data: number[] | string[]) => {
  const poseidon = await buildPoseidon();
  const hash = poseidon(data);

  return poseidon.F.toString(hash);
};
