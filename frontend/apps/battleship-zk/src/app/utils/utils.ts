import { AlertContainerFactory, withoutCommas } from '@gear-js/react-hooks';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { ACCOUNT_ID_LOCAL_STORAGE_KEY } from '@/app/consts';
import { HexString } from '@polkadot/util/types';

export function formatDate(input: string | number): string {
  const date = new Date(input);
  return date.toLocaleDateString('en-US', {
    month: 'long',
    day: 'numeric',
    year: 'numeric',
  });
}

// Set value in seconds
export const sleep = (s: number) => new Promise((resolve) => setTimeout(resolve, s * 1000));

export const copyToClipboard = async ({
  alert,
  value,
  successfulText,
}: {
  alert?: AlertContainerFactory;
  value: string;
  successfulText?: string;
}) => {
  const onSuccess = () => {
    if (alert) {
      alert.success(successfulText || 'Copied');
    }
  };
  const onError = () => {
    if (alert) {
      alert.error('Copy error');
    }
  };

  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
      document.execCommand('copy');
      onSuccess();
    } catch (err) {
      console.error('Unable to copy to clipboard', err);
      onError();
    }
    document.body.removeChild(textArea);
  }

  if (window.isSecureContext && navigator.clipboard) {
    navigator.clipboard
      .writeText(value)
      .then(() => onSuccess())
      .catch(() => onError());
  } else {
    unsecuredCopyToClipboard(value);
  }
};

export const isLoggedIn = ({ address }: InjectedAccountWithMeta) =>
  localStorage.getItem(ACCOUNT_ID_LOCAL_STORAGE_KEY) === address;

export function prettyDate(
  input: number | Date | string,
  options: Intl.DateTimeFormatOptions = {
    dateStyle: 'long',
    timeStyle: 'short',
    hourCycle: 'h23',
  },
  locale: string = 'en-US',
) {
  const date = typeof input === 'string' ? new Date(input) : input;
  return new Intl.DateTimeFormat(locale, options).format(date);
}

export function trimEndSlash(url: string): string {
  return url.endsWith('/') ? url.slice(0, -1) : url;
}

export const prettyAddress = (address: HexString) => {
  return address.slice(0, 6) + '...' + address.slice(-4);
};

export function toNumber(value: string) {
  return +withoutCommas(value);
}
