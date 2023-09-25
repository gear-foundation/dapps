import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'simple-nft-contract-address',
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};

export const ADDRESS = {
  NODE: process.env.REACT_APP_NODE as string,
  FACTORY: process.env.REACT_APP_FACTORY_ADDRESS as HexString,
  DEFAULT_NODES: process.env.REACT_APP_DEFAULT_NODES_URL as string,
  DEFAULT_CONTRACT: process.env.REACT_APP_DEFAULT_CONTRACT_ADDRESS as HexString,
  DEFAULT_TESTNET_CONTRACT: process.env.REACT_APP_DEFAULT_TESTNET_CONTRACT_ADDRESS as HexString,
  IPFS: process.env.REACT_APP_IPFS_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
};
