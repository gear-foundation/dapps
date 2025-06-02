import { atom } from 'jotai';

export const IS_BALANCE_LOW_ATOM = atom<boolean>(false);

export const isPendingUI = atom<boolean>(false);

export const ENV = {
  DEFAULT_NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  IPFS_GATEWAY: import.meta.env.VITE_IPFS_GATEWAY_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  EXPLORER_URL: import.meta.env.VITE_NFT_EXPLORER_URL as string,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN,
};
