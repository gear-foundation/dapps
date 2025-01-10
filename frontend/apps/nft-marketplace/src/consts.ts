import { HexString } from '@polkadot/util/types';

const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  IPFS: process.env.REACT_APP_IPFS_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  MARKETPLACE_CONTRACT: process.env.REACT_APP_MARKETPLACE_CONTRACT_ADDRESS as HexString,
  NFT_CONTRACT: process.env.REACT_APP_NFT_CONTRACT_ADDRESS as HexString,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

const MIN_PRICE = 1000000000000;

export { ADDRESS, LOCAL_STORAGE, MIN_PRICE };
