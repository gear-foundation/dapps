import { HexString } from '@polkadot/util/types';

const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  GASLESS_API: process.env.REACT_APP_API_ADDRESS as string,
  CONTRACT: process.env.REACT_APP_CONTRACT_ADDRESS as HexString,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

const ROUTES = {
  HOME: '/',
  LEADERBOARD: '/leaderboard',
};

export { ADDRESS, LOCAL_STORAGE, ROUTES };
