import { getViteEnv } from './utils';

function logPublicEnvs(customValues: Record<string, string> = {}) {
  const node = getViteEnv('NODE_ADDRESS');
  const contract = getViteEnv('CONTRACT_ADDRESS');
  const code = getViteEnv('CODE_ADDRESS');
  const ipfs = getViteEnv('IPFS_ADDRESS');
  const ipfsGateway = getViteEnv('IPFS_GATEWAY_ADDRESS');

  Object.entries({ ...customValues, node, contract, code, ipfs, ipfsGateway }).forEach(
    ([key, value]) => value && console.log(`${key}:`, value),
  );
}

export { logPublicEnvs };
