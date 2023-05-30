const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
  NODE: 'node',
  NODES: 'nodes',
  CONTRACT_ADDRESS: 'simple-nft-contract-address',
};

const ADDRESS = {
  NODE: localStorage[LOCAL_STORAGE.NODE] || (process.env.REACT_APP_NODE_ADDRESS as string),
  DEFAULT_NODES: process.env.REACT_APP_DEFAULT_NODES_URL as string,
};

export { ADDRESS, LOCAL_STORAGE };
