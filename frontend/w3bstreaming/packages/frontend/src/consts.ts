import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'straming-contract-address',
};

export const ADDRESS = {
  NODE: (process.env.REACT_APP_STREAMING_NODE || 'wss://testnet.vara-network.io') as string,
  NODES: 'https://idea.gear-tech.io/gear-nodes',
  CONTRACT: (process.env.REACT_APP_STREAMING_PROGRAM_ADDRESS ||
    '0xd564fdc32fb4d5d288fd78575f0e8e88c0ff91eb055f9a4a825a54ab222001af') as HexString,
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};
