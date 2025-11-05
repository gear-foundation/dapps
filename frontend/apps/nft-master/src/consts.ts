import { atom } from 'jotai';

export const IS_BALANCE_LOW_ATOM = atom<boolean>(false);

export const isPendingUI = atom<boolean>(false);

export const ENV = {
  DEFAULT_NODE: import.meta.env.VITE_NODE_ADDRESS,
  IPFS_GATEWAY: import.meta.env.VITE_IPFS_GATEWAY_ADDRESS,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL,
  DNS_NAME: import.meta.env.VITE_DNS_NAME,
  EXPLORER_URL: import.meta.env.VITE_NFT_EXPLORER_URL,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN,
};
