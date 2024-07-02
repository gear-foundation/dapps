import { atom } from 'jotai';

export const IS_BALANCE_LOW_ATOM = atom<boolean>(false);

export const isPendingUI = atom<boolean>(false);

export const ADDRESS = {
  DEFAULT_NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  DNS_API_URL: process.env.REACT_APP_DNS_API_URL as string,
  DNS_NAME: process.env.REACT_APP_DNS_NAME as string,
  EXPLORER_URL: process.env.REACT_APP_NFT_EXPLORER_URL as string,
  SENTRY_DSN: process.env.REACT_APP_SENTRY_DSN,
};
