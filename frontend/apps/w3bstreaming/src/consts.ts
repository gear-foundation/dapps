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
  CONTRACT: (process.env.REACT_APP_CONTRACT_ADDRESS ||
    '0x0658601d3de26a2172cdbc13543250dde44788c9b04636b1679c0bb5b71cf23e') as HexString,
  IPFS_GATEWAY: 'https://ipfs-gw.gear-tech.io/ipfs' as string,
  IPFS_NODE: process.env.REACT_APP_IPFS_ADDRESS as string,
  SIGNALING_SERVER: process.env.REACT_APP_SIGNALING_SERVER || 'ws://127.0.0.1:3001',
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};
