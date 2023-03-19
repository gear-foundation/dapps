import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export const createLauncheInitial = {
  fuel: 0,
  payload: 0
};

export const ENV = {
  store: process.env.REACT_APP_STORE_ADDRESS as HexString,
  balance: process.env.REACT_APP_FT_ADDRESS as HexString,
  contract: process.env.REACT_APP_CONTRACT_ADDRESS as HexString,
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
};

export const WEATHER = [
  'Sunny ☀️',
  'Cloudy ☁️',
  'Rainy 🌧',
  'Storm 🌩',
  'Thunder ⛈',
  'Tornado 🌪'
]