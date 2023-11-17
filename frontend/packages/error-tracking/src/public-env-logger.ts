import { getCRAEnv, getViteEnv } from './utils';

function logPublicEnvs(customValues: Record<string, string> = {}) {
  const node = getCRAEnv('NODE_ADDRESS') || getViteEnv('NODE_ADDRESS');
  const contract = getCRAEnv('CONTRACT_ADDRESS') || getViteEnv('CONTRACT_ADDRESS');
  const code = getCRAEnv('CODE_ADDRESS') || getViteEnv('CODE_ADDRESS');
  const ipfs = getCRAEnv('IPFS_ADDRESS') || getViteEnv('IPFS_ADDRESS');
  const ipfsGateway = getCRAEnv('IPFS_GATEWAY_ADDRESS') || getViteEnv('IPFS_GATEWAY_ADDRESS');

  Object.entries({ ...customValues, node, contract, code, ipfs, ipfsGateway }).forEach(
    ([key, value]) => value && console.log(`${key}:`, value),
  );
}

export { logPublicEnvs };
