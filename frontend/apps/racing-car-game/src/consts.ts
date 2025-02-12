export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'simple-nft-contract-address',
};

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS as string,
  BASE_NODES: import.meta.env.VITE_DEFAULT_NODES_URL as string,
  STAGING_NODES: import.meta.env.VITE_STAGING_NODES_URL as string,
  SENTRY_DSN: import.meta.env.VITE_SENTRY_DSN_CARS as string,
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['StartGame', 'Move', 'Skip'];
