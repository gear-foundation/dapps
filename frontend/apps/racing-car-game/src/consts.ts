export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'simple-nft-contract-address',
};

export const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS,
  DNS_API_URL: process.env.REACT_APP_DNS_API_URL as string,
  DNS_NAME: process.env.REACT_APP_DNS_NAME as string,
  GASLESS_BACKEND: process.env.REACT_APP_GASLESS_BACKEND_ADDRESS as string,
  BASE_NODES: process.env.REACT_APP_DEFAULT_NODES_URL as string,
  STAGING_NODES: process.env.REACT_APP_STAGING_NODES_URL as string,
  SENTRY_DSN: process.env.REACT_APP_SENTRY_DSN_CARS as string,
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['StartGame', 'Move', 'Skip'];
