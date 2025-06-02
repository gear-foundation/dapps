const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  IPFS: import.meta.env.VITE_IPFS_ADDRESS as string,
  IPFS_GATEWAY: import.meta.env.VITE_IPFS_GATEWAY_ADDRESS as string,
  CONTRACT_ADDRESS: import.meta.env.VITE_CONTRACT_ADDRESS as `0x${string}`,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

const FILTERS = ['All', 'My', 'Approved'];

export { ENV, LOCAL_STORAGE, FILTERS };
