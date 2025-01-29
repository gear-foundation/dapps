import { HexString } from '@polkadot/util/types';

const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  IPFS: import.meta.env.VITE_IPFS_ADDRESS as string,
  IPFS_GATEWAY: import.meta.env.VITE_IPFS_GATEWAY_ADDRESS as string,
  MARKETPLACE_CONTRACT: import.meta.env.VITE_MARKETPLACE_CONTRACT_ADDRESS as HexString,
  NFT_CONTRACT: import.meta.env.VITE_NFT_CONTRACT_ADDRESS as HexString,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

const MIN_PRICE = 1000000000000;

export { ADDRESS, LOCAL_STORAGE, MIN_PRICE };
