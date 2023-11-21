import clsx from 'clsx';
import { useEffect } from 'react';
import { useLocation } from 'react-router';
import { ADDRESS } from './consts';

export const cx = (...styles: string[]) => clsx(...styles);

export const copyToClipboard = (value: string) =>
  navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

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
