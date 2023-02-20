import { HexString } from '@polkadot/util/types';

const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  FT_CONTRACT: '0xa2677f49725647da5cff15e8a42b2ead9102c387d646ff856f586b81e4b598a0' as HexString,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

export { ADDRESS, LOCAL_STORAGE };
