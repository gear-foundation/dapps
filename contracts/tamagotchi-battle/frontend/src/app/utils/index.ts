import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'app/consts';

export const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

export const toSeconds = (n: number) => {
  const N = Math.abs(n);
  return N < 10 ? `0${N}` : `${N}`;
};
