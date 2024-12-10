import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'straming-contract-address',
};

export const ADDRESS = {
  NODE: (process.env.REACT_APP_NODE_ADDRESS || 'wss://testnet.vara-network.io') as string,
  NODES: 'https://idea.gear-tech.io/gear-nodes',
  DNS_API_URL: process.env.REACT_APP_DNS_API_URL as string,
  DNS_NAME: process.env.REACT_APP_DNS_NAME as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  IPFS_NODE: process.env.REACT_APP_IPFS_ADDRESS as string,
  SIGNALING_SERVER: process.env.REACT_APP_SIGNALING_SERVER || 'ws://127.0.0.1:3001',
  BACKEND_SERVER: process.env.REACT_APP_BACKEND_SERVER || 'http://127.0.0.1:3001',
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};
