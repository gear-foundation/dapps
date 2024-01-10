import { signatureVerify } from '@polkadot/util-crypto';
import { Socket } from 'socket.io';

export const isValidSig = (publicKey: string, msg: string): boolean => {
  try {
    const result = signatureVerify(publicKey, msg, publicKey);
    return result.isValid;
  } catch (error) {
    return false;
  }
};

export const getConnections = (
  connections: Map<string, { [userId: string]: Socket }>
) => {
  const f = {} as any;
  const streamKeys = connections.keys();
  for (let streamKey of streamKeys) {
    const keys = Object.keys(connections.get(streamKey) || {});
    f[streamKey] = keys.reduce(
      (acc, item) => ({
        ...acc,
        [item]: String(connections.get(streamKey)?.[item]),
      }),
      {}
    );
  }

  return f;
};
