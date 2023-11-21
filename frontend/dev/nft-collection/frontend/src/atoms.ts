import { atom } from 'jotai';
import { Account } from '@gear-js/react-hooks/dist/esm/types';
import { ADDRESS } from './consts';

export const CONTRACT_ADDRESS_ATOM = atom<string>(ADDRESS.FACTORY);

export const ACCOUNT_ATOM = atom<Account | null>(null);

export const IPFS_ATOM = atom({
  address: ADDRESS.IPFS,
  gateway: ADDRESS.IPFS_GATEWAY,
});
