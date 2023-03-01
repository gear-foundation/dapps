import { AlertContainerFactory } from '@gear-js/react-hooks';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'app/consts';

export const copyToClipboard = (key: string, alert: AlertContainerFactory, successfulText?: string) => {
  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
      document.execCommand('copy');
      alert.success(successfulText || 'Copied');
    } catch (err) {
      console.error('Unable to copy to clipboard', err);
      alert.error('Copy error');
    }
    document.body.removeChild(textArea);
  }

  if (window.isSecureContext && navigator.clipboard) {
    navigator.clipboard
      .writeText(key)
      .then(() => alert.success(successfulText || 'Copied'))
      .catch(() => alert.error('Copy error'));
  } else {
    unsecuredCopyToClipboard(key);
  }
};

export const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

export const toSeconds = (n: number) => {
  const N = Math.abs(n);
  return N < 10 ? `0${N}` : `${N}`;
};
