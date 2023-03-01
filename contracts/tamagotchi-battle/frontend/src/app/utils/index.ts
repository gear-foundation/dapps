import { AlertContainerFactory } from '@gear-js/react-hooks';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'app/consts';

export const copyToClipboard = (key: string, alert: AlertContainerFactory, successfulText?: string) => {
  navigator.clipboard
    .writeText(key)
    .then(() => alert.success(successfulText || 'Copied'))
    .catch(() => alert.error('Copy error'));
};
export const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

export const toSeconds = (n: number) => {
  const N = Math.abs(n);
  return N < 10 ? `0${N}` : `${N}`;
};
