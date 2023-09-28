import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'simple-nft-contract-address',
};

export const ADDRESS = {
  NODE: 'wss://node-workshop.gear.rs:443',
  NODES: 'https://idea.gear-tech.io/gear-nodes',
  CONTRACT: '0x5c08daae9aab3ecb0bf348fe318b580059436dbdaaaba9ac676b89ac02a58954' as HexString,
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};
