import clsx from 'clsx';
import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';

export const cx = (...styles: string[]) => clsx(...styles);

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
