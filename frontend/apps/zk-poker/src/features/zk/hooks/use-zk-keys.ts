import { atom, useAtomValue } from 'jotai';

import { keyGen } from '../lib';

const ZK_PAIR_KEY = 'zk_keys';

type Keys<T> = {
  sk: T;
  pk: { x: T; y: T; z: T };
};

const transformKeys = <T, R>(keys: Keys<T>, transform: (x: T) => R) => ({
  sk: transform(keys.sk),
  pk: {
    x: transform(keys.pk.x),
    y: transform(keys.pk.y),
    z: transform(keys.pk.z),
  },
});

const generateKeys = (): Keys<bigint> => {
  const { pk, sk } = keyGen(64);
  const { X: x, Y: y, Z: z } = pk;
  const bigintKeys = { sk, pk: { x, y, z } };
  const stringKeys = transformKeys(bigintKeys, (value) => String(value));
  localStorage.setItem(ZK_PAIR_KEY, JSON.stringify(stringKeys));

  return bigintKeys;
};

const zkKeysAtom = atom(() => {
  const stored = localStorage.getItem(ZK_PAIR_KEY);

  if (!stored) return generateKeys();

  const parsed = JSON.parse(stored) as Keys<string>;

  return {
    sk: BigInt(parsed.sk),
    pk: {
      x: BigInt(parsed.pk.x),
      y: BigInt(parsed.pk.y),
      z: BigInt(parsed.pk.z),
    },
  };
});

const useZkKeys = () => useAtomValue(zkKeysAtom);

export { useZkKeys };
