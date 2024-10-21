import { HexString } from '@polkadot/util/types';

const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  CONTRACT: process.env.REACT_APP_CONTRACT_ADDRESS as HexString,
  FT_CONTRACT: process.env.REACT_APP_FT_CONTRACT_ADDRESS as HexString,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

const periods = [
  { label: 'Year', value: 'Year', rate: 12 },
  { label: '9 months', value: 'NineMonths', rate: 9 },
  { label: '6 months', value: 'SixMonths', rate: 6 },
  { label: '3 months', value: 'ThreeMonths', rate: 3 },
  { label: '1 month', value: 'Month', rate: 1 },
];

const initialValues = { isRenewal: true, period: periods[0].value };

const VOUCHER_MIN_LIMIT = 18;

export { ADDRESS, LOCAL_STORAGE, periods, initialValues, VOUCHER_MIN_LIMIT };
