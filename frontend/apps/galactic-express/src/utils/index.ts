import clsx from 'clsx';

export const cx = (...styles: string[]) => clsx(...styles);

export const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
  navigator.userAgent,
);

export const getPanicType = (error: unknown) => {
  if (error instanceof Error) {
    const errorWords = error?.message?.replaceAll("'", '').replaceAll('"', '').trim().split(' ');
    const panicType = errorWords[errorWords.length - 1];

    return panicType;
  }

  return null;
};
