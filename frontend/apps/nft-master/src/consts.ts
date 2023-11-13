import { atom } from 'jotai';
import { HexString } from '@polkadot/util/types';

export const IS_BALANCE_LOW_ATOM = atom<boolean>(false);

export const isPendingUI = atom<boolean>(false);

export const ADDRESS = {
  DEFAULT_NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  IPFS_GATEWAY: process.env.REACT_APP_IPFS_GATEWAY_ADDRESS as string,
  MASTER_CONTRACT: process.env.REACT_APP_CONTRACT_ADDRESS as HexString,
  GAME_STATE_SOCKET: process.env.REACT_APP_NFT_STATE_SOCKET as string,
  EXPLORER_URL: process.env.REACT_APP_NFT_EXPLORER_URL as string,
  SENTRY_DSN: process.env.REACT_APP_SENTRY_DSN,
};
