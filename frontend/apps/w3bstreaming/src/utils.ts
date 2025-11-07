import clsx from 'clsx';
import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { Socket, io } from 'socket.io-client';

import { ENV } from './consts';

export const cx = (...styles: string[]) => clsx(...styles);

export const socket: Socket = io(ENV.SIGNALING_SERVER);

export function useScrollToTop() {
  const { pathname } = useLocation();

  useEffect(() => {
    document.documentElement.scrollTo(0, 0);
  }, [pathname]);
}

export const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
  navigator.userAgent,
);

export const arrayToRecord = <K extends string, V>(array: Array<[K, V]>): Record<K, V> => {
  return array.reduce<Record<K, V>>(
    (record, [key, value]) => {
      record[key] = value;
      return record;
    },
    {} as Record<K, V>,
  );
};
