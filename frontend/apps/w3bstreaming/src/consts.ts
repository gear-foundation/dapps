export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'straming-contract-address',
};

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  NODES: import.meta.env.VITE_NODES_API_URL as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  IPFS_GATEWAY: import.meta.env.VITE_IPFS_GATEWAY_ADDRESS as string,
  IPFS_NODE: import.meta.env.VITE_IPFS_ADDRESS as string,
  SIGNALING_SERVER: import.meta.env.VITE_SIGNALING_SERVER as string,
  BACKEND_SERVER: import.meta.env.VITE_BACKEND_SERVER as string,
};

export const SEARCH_PARAMS = {
  MASTER_CONTRACT_ID: 'master',
};
