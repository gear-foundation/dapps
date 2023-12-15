import { AlertContainerFactory } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { useEffect } from 'react';
import { useLocation } from 'react-router';
import { Socket, io } from 'socket.io-client';
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

export const socket: Socket = io(ADDRESS.SIGNALING_SERVER);

export function ScrollToTop() {
  const { pathname } = useLocation();

  useEffect(() => {
    document.documentElement.scrollTo({
      top: 0,
      left: 0,
    });
  }, [pathname]);

  return null;
}

export const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
  navigator.userAgent,
);

export const logger = (message: unknown | unknown[]) => {
  const date = new Date();
  let milliseconds = '';
  const milli = date.getMilliseconds();

  if (milli < 10) {
    milliseconds = `00${milli}`;
  } else if (milli < 100) {
    milliseconds = `0${milli}`;
  } else {
    milliseconds = `${milli}`;
  }

  const time = `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}.${milliseconds}`;

  console.log(time, message);
};
