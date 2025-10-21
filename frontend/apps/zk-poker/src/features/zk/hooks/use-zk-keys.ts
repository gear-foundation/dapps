import { atom, useAtomValue } from 'jotai';

import { keyGen } from '../lib';

const ZK_PAIR_KEY = 'zk_keys';

type Keys<T> = {
  sk: T;
  pk: { X: T; Y: T; Z: T };
};

const transformKeys = <T, R>(keys: Keys<T>, transform: (x: T) => R) => ({
  sk: transform(keys.sk),
  pk: {
    X: transform(keys.pk.X),
    Y: transform(keys.pk.Y),
    Z: transform(keys.pk.Z),
  },
});

const generateKeys = (): Keys<bigint> => {
  const { pk, sk } = keyGen(64);
  const bigintKeys = { sk, pk };
  const stringKeys = transformKeys(bigintKeys, (value) => String(value));
  localStorage.setItem(ZK_PAIR_KEY, JSON.stringify(stringKeys));

  return bigintKeys;
};

const zkKeysAtom = atom(() => {
  const stored = localStorage.getItem(ZK_PAIR_KEY);

  if (!stored) return generateKeys();

  const parsed = JSON.parse(stored) as Keys<string>;

  if (!parsed.sk || !parsed.pk.X || !parsed.pk.Y || !parsed.pk.Z) return generateKeys();

  return {
    sk: BigInt(parsed.sk),
    pk: {
      X: BigInt(parsed.pk.X),
      Y: BigInt(parsed.pk.Y),
      Z: BigInt(parsed.pk.Z),
    },
  };
});

const useZkKeys = () => useAtomValue(zkKeysAtom);

export { useZkKeys };
