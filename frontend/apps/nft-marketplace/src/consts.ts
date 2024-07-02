import { HexString } from '@polkadot/util/types';

const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  IPFS: process.env.REACT_APP_IPFS_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  DNS_API_URL: process.env.REACT_APP_DNS_API_URL as string,
  DNS_NAME: process.env.REACT_APP_DNS_NAME as string,
  NFT_CONTRACT: process.env.REACT_APP_NFT_CONTRACT_ADDRESS as HexString,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export { ADDRESS, LOCAL_STORAGE };
