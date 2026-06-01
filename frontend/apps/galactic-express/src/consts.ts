const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  DNS_CONTRACT_ADDRESS: import.meta.env.VITE_DNS_CONTRACT_ADDRESS as `0x${string}`,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

const ROUTES = {
  HOME: '/',
  LOGIN: 'login',
  NOT_AUTHORIZED: 'not-authorized',
};

export { ENV, LOCAL_STORAGE, ROUTES };
