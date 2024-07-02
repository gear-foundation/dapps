const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  DNS_API_URL: process.env.REACT_APP_DNS_API_URL as string,
  DNS_NAME: process.env.REACT_APP_DNS_NAME as string,
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

export { ADDRESS, LOCAL_STORAGE, ROUTES };
