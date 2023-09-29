import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'simple-nft-contract-address',
};

export const ADDRESS = {
  NODE: 'wss://vit.vara-network.io',
  NODES: 'https://idea.gear-tech.io/gear-nodes',
  CONTRACT: '0xb5f44ec900f394488fcddf9024ec3af58f52c797ba745a0dc91d99173650a91d' as HexString,
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};
