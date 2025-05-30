import { atom, useAtomValue } from 'jotai';

import { keyGen } from '../lib';

const ZK_PAIR_KEY = 'zk_keys';

type Keys<T = Uint8Array> = {
  sk: T;
  pk: { x: T; y: T; z: T };
};

const bigintToBytes32LE = (x: bigint): Uint8Array => {
  const bytes = Uint8Array.from(Buffer.from(x.toString(16).padStart(64, '0'), 'hex'));
  return new Uint8Array([...bytes].reverse());
};

const uint8ArrayToHex = (arr: Uint8Array): string => {
  return Array.from(arr)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
};

const hexToUint8Array = (hex: string): Uint8Array => {
  return new Uint8Array(hex.match(/.{1,2}/g)?.map((byte) => parseInt(byte, 16)) || []);
};

const transformKeys = <T, R>(keys: Keys<T>, transform: (x: T) => R) => ({
  sk: transform(keys.sk),
  pk: {
    x: transform(keys.pk.x),
    y: transform(keys.pk.y),
    z: transform(keys.pk.z),
  },
});

const generateKeys = (): Keys => {
  const { pk, sk } = keyGen(64);
  const { X: x, Y: y, Z: z } = pk;

  const keys = transformKeys({ sk, pk: { x, y, z } }, bigintToBytes32LE);

  localStorage.setItem(ZK_PAIR_KEY, JSON.stringify(transformKeys(keys, uint8ArrayToHex)));

  return keys;
};

const zkKeysAtom = atom(() => {
  const stored = localStorage.getItem(ZK_PAIR_KEY);
  if (!stored) return generateKeys();

  const parsed = JSON.parse(stored) as Keys<string>;
  return {
    sk: hexToUint8Array(parsed.sk),
    pk: {
      x: hexToUint8Array(parsed.pk.x),
      y: hexToUint8Array(parsed.pk.y),
      z: hexToUint8Array(parsed.pk.z),
    },
  };
});

const useKeys = () => useAtomValue(zkKeysAtom);

export { useKeys };
