const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
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
