import clsx from 'clsx';
import { useEffect } from 'react';
import { useLocation } from 'react-router';
import { AlertContainerFactory } from '@gear-js/react-hooks/dist/esm/types';
import { ADDRESS } from './consts';

export const cx = (...styles: string[]) => clsx(...styles);

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

export const shortenString = (str: string, length: number): string => `${str.slice(0, length)}...${str.slice(-length)}`;

export const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid}`;

function ScrollToTop() {
  const { pathname } = useLocation();

  useEffect(() => {
    document.documentElement.scrollTo({
      top: 0,
      left: 0,
    });
  }, [pathname]);

  return null;
}

export { ScrollToTop };
