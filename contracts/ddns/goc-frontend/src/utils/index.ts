import { Hex } from '@gear-js/api';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { AlertContainerFactory } from '@gear-js/react-hooks';
import { LOCAL_STORAGE } from 'consts';
import { getStatus, getCountdown } from './status';

const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

const isWinner = (value: Hex) => !value.startsWith('0x00');

const getNumber = (value: string) => +value.replaceAll(',', '');
const getDate = (value: number) => new Date(value).toLocaleString();

const copyToClipboard = async (key: string, alert: AlertContainerFactory, successfulText?: string) => {
  try {
    await navigator.clipboard.writeText(key);
    alert.success(successfulText || 'Copied');
  } catch (err) {
    alert.error('Copy error');
  }
};

export { isLoggedIn, isWinner, getNumber, getDate, getStatus, getCountdown, copyToClipboard };
